use crate::infrastructure::lua;
use crate::infrastructure::slint::aura_handle::{new_command_queue, CommandQueue};
use crate::infrastructure::wayland::{AuraSurface, WlState};
use crate::usecase::render::{RenderError, Renderer};
use slint::platform::software_renderer::{
    LineBufferProvider, MinimalSoftwareWindow, RepaintBufferType,
};
use slint_interpreter::{Compiler, ComponentHandle, ComponentInstance, SharedString, Value};
use smithay_client_toolkit::reexports::client::QueueHandle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::{error, info, warn};
use waio_shared::entity::Aura;

pub struct AuraInstance {
    pub instance: ComponentInstance,
    pub window: Rc<MinimalSoftwareWindow>,
    pub surface: AuraSurface,
}

struct FrameBuffer<'a> {
    frame_buffer: &'a mut [u8],
    stride: usize,
}

impl<'a> LineBufferProvider for FrameBuffer<'a> {
    type TargetPixel = slint::platform::software_renderer::Rgb8Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        let line_begin = line * self.stride;
        let slice_start = (line_begin + range.start) * 4;
        let slice_end = (line_begin + range.end) * 4;
        if slice_end <= self.frame_buffer.len() {
            let pixel_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    self.frame_buffer[slice_start..slice_end].as_mut_ptr()
                        as *mut slint::platform::software_renderer::Rgb8Pixel,
                    range.len(),
                )
            };
            render_fn(pixel_slice);
        }
    }
}

pub struct SlintRenderer {
    pub compositor: smithay_client_toolkit::compositor::CompositorState,
    pub qh: QueueHandle<WlState>,
    auras: Rc<RefCell<HashMap<String, AuraInstance>>>,
    lua_state: Rc<RefCell<mlua::Lua>>,
    command_queue: CommandQueue,
}

impl SlintRenderer {
    pub fn new(
        compositor: smithay_client_toolkit::compositor::CompositorState,
        qh: QueueHandle<WlState>,
    ) -> Self {
        let lua = mlua::Lua::new();
        let _ = lua::register_all(&lua);
        Self {
            compositor,
            qh,
            auras: Rc::new(RefCell::new(HashMap::new())),
            lua_state: Rc::new(RefCell::new(lua)),
            command_queue: new_command_queue(),
        }
    }

    fn compile(&self, code: &str) -> Result<slint_interpreter::ComponentDefinition, RenderError> {
        let compiler = Compiler::default();
        let result = spin_on::spin_on(
            compiler.build_from_source(code.to_string(), std::path::PathBuf::from("virtual.slint")),
        );
        for diag in result.diagnostics() {
            match diag.level() {
                slint_interpreter::DiagnosticLevel::Error => error!("Slint: {}", diag),
                slint_interpreter::DiagnosticLevel::Warning => warn!("Slint: {}", diag),
                _ => info!("Slint: {}", diag),
            }
        }
        let definition = result
            .component("AuraBar")
            .or_else(|| result.components().next())
            .ok_or_else(|| {
                let names: Vec<_> = result.components().map(|c| c.name().to_string()).collect();
                RenderError::SlintCompilationFailed(format!(
                    "No component found. Available: {:?}",
                    names
                ))
            })?;
        info!("Found Slint component: {}", definition.name());
        Ok(definition)
    }

    /// Process commands from Lua queue - called from main thread
    pub fn process_commands(&self) -> Result<(), RenderError> {
        let commands = {
            let mut q = self
                .command_queue
                .lock()
                .map_err(|e| RenderError::RenderFailed(format!("queue lock: {}", e)))?;
            std::mem::take(&mut *q)
        };
        if commands.is_empty() {
            return Ok(());
        }
        let mut auras = self.auras.borrow_mut();
        for cmd in commands {
            if let Some(aura_inst) = auras.get_mut(&cmd.aura_id) {
                info!("Applying: {} = {}", cmd.property, cmd.value);
                let _ = aura_inst
                    .instance
                    .set_property(&cmd.property, Value::String(SharedString::from(&cmd.value)));
                aura_inst.window.request_redraw();
            }
        }
        Ok(())
    }

    /// Redraw all auras
    pub fn redraw_all(&self) -> Result<(), RenderError> {
        let mut auras = self.auras.borrow_mut();
        for (_, aura_inst) in auras.iter_mut() {
            aura_inst
                .surface
                .draw(|canvas, w, _h| {
                    let mut frame_buffer = FrameBuffer {
                        frame_buffer: canvas,
                        stride: w as usize,
                    };
                    aura_inst.window.draw_if_needed(|renderer| {
                        renderer.render_by_line(&mut frame_buffer);
                    });
                })
                .map_err(|e| RenderError::RenderFailed(e.to_string()))?;
        }
        Ok(())
    }

    pub fn render_aura_with_state(
        &self,
        aura: &Aura,
        wl_state: &mut WlState,
    ) -> Result<(), RenderError> {
        info!("Rendering aura: {}", aura.name);

        let definition = self.compile(&aura.slint_code)?;

        // 1. Create MinimalSoftwareWindow
        let window = Rc::new(MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer));
        window.set_size(slint::PhysicalSize::new(
            aura.config.size.width,
            aura.config.size.height,
        ));

        // 2. Create instance bound to window
        let instance = definition
            .create_with_window(window.clone())
            .map_err(|e| RenderError::RenderFailed(format!("create_with_window: {}", e)))?;

        // 3. Set initial properties
        let _ = instance.set_property("time_text", Value::String("00:00:00".into()));
        let _ = instance.set_property("date_text", Value::String("2026-01-01".into()));

        // 4. Run Lua code with command queue bridge
        if let Some(ref code) = aura.lua_code {
            let lua = self.lua_state.borrow();
            lua::slint_bridge::register_with_queue(
                &lua,
                aura.id.clone(),
                self.command_queue.clone(),
            )?;
            match lua.load(code).exec() {
                Ok(_) => info!("Lua code executed successfully"),
                Err(e) => error!("Lua execution error: {}", e),
            }
        }

        // 5. Create Wayland surface
        let mut surface = AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            &aura.config,
        )
        .map_err(|e| RenderError::WaylandError(e.to_string()))?;

        // 6. Initial render
        window.request_redraw();
        surface
            .draw(|canvas, w, _h| {
                let mut frame_buffer = FrameBuffer {
                    frame_buffer: canvas,
                    stride: w as usize,
                };
                window.draw_if_needed(|renderer| {
                    renderer.render_by_line(&mut frame_buffer);
                });
            })
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        // 7. Store
        let mut auras = self.auras.borrow_mut();
        auras.insert(
            aura.id.clone(),
            AuraInstance {
                instance,
                window,
                surface,
            },
        );

        info!("Aura '{}' rendered successfully", aura.name);
        Ok(())
    }
}

impl Renderer for SlintRenderer {
    fn init(&self) -> Result<(), RenderError> {
        info!("SlintRenderer initialized");
        Ok(())
    }

    fn render_aura(&self, _aura: &Aura) -> Result<(), RenderError> {
        Err(RenderError::RenderFailed(
            "Use render_aura_with_state instead".into(),
        ))
    }

    fn remove_aura(&self, aura_id: &str) -> Result<(), RenderError> {
        let mut auras = self.auras.borrow_mut();
        if auras.remove(aura_id).is_some() {
            Ok(())
        } else {
            Err(RenderError::AuraNotFound(aura_id.to_string()))
        }
    }

    fn update_aura(&self, _aura_id: &str, _slint_code: &str) -> Result<(), RenderError> {
        Err(RenderError::RenderFailed(
            "Use render_aura_with_state instead".into(),
        ))
    }

    fn shutdown(&self) -> Result<(), RenderError> {
        let mut auras = self.auras.borrow_mut();
        auras.clear();
        Ok(())
    }
}
