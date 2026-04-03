#![allow(dead_code)]

use crate::infrastructure::lua;
use crate::infrastructure::slint::aura_handle::{new_command_queue, CommandQueue};
use crate::infrastructure::slint::software_renderer::SoftwareRenderHandler;
use crate::infrastructure::wayland::{AuraSurface, WlState};
use crate::usecase::render::{RenderError, Renderer};
use slint_interpreter::Compiler;
use smithay_client_toolkit::reexports::client::QueueHandle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::{error, info, warn};
use waio_shared::entity::Aura;

pub struct AuraInstance {
    pub surface: AuraSurface,
}

pub struct SlintRenderer {
    pub compositor: smithay_client_toolkit::compositor::CompositorState,
    pub qh: QueueHandle<WlState>,
    auras: Rc<RefCell<HashMap<String, AuraInstance>>>,
    lua_state: Rc<RefCell<mlua::Lua>>,
    command_queue: CommandQueue,
    software_handler: Rc<RefCell<SoftwareRenderHandler>>,
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
            software_handler: Rc::new(RefCell::new(SoftwareRenderHandler::new())),
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

        let handler = self.software_handler.borrow();
        for cmd in commands {
            info!("Sending property update: {} = {}", cmd.property, cmd.value);
            handler.set_property(&cmd.property, &cmd.value);
        }
        Ok(())
    }

    /// Redraw all auras
    pub fn redraw_all(&self) -> Result<(), RenderError> {
        let render_result = {
            let handler = self.software_handler.borrow();
            handler.render()
        };

        let mut auras = self.auras.borrow_mut();
        for (_, aura_inst) in auras.iter_mut() {
            tracing::debug!(
                "Canvas size: {}x{} ({} bytes expected)",
                aura_inst.surface.width,
                aura_inst.surface.height,
                aura_inst.surface.width * aura_inst.surface.height * 4
            );

            let render_result = render_result.clone();

            aura_inst
                .surface
                .draw(move |canvas, canvas_w, canvas_h| {
                    tracing::debug!(
                        "draw callback: canvas len={}, w={}, h={}",
                        canvas.len(),
                        canvas_w,
                        canvas_h
                    );

                    match &render_result {
                        Ok(src) => {
                            if src.is_empty() {
                                canvas.fill(0xCC);
                                tracing::debug!("Empty render - placeholder");
                                return;
                            }
                            let src_stride = canvas_w as usize * 4;
                            let dst_stride = canvas_w as usize * 4;
                            let height = canvas_h as usize;
                            let width = canvas_w as usize;

                            for y in 0..height.min(canvas.len() / (canvas_w as usize * 4)) {
                                for x in 0..width.min(canvas.len() / 4) {
                                    let src_idx = y * src_stride + x * 4;
                                    let dst_idx = y * dst_stride + x * 4;
                                    if dst_idx + 3 < canvas.len() && src_idx + 3 < src.len() {
                                        canvas[dst_idx + 0] = src[src_idx + 2]; // B
                                        canvas[dst_idx + 1] = src[src_idx + 1]; // G
                                        canvas[dst_idx + 2] = src[src_idx + 0]; // R
                                        canvas[dst_idx + 3] = src[src_idx + 3]; // A
                                    }
                                }
                            }
                            tracing::debug!("Render complete");
                        }
                        Err(e) => {
                            tracing::error!("Render error: {}", e);
                            canvas.fill(0xAA);
                        }
                    }
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

        {
            let mut handler = self.software_handler.borrow_mut();
            handler.create_component(
                &aura.slint_code,
                aura.config.size.width,
                aura.config.size.height,
            )?;
        }

        self.software_handler
            .borrow()
            .set_property("time_text", "00:00:00");
        self.software_handler
            .borrow()
            .set_property("date_text", "2026-01-01");

        // Run Lua code with command queue bridge
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

        // Create Wayland surface
        let mut surface = AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            &aura.config,
        )
        .map_err(|e| RenderError::WaylandError(e.to_string()))?;

        surface
            .draw(|canvas, _w, _h| {
                canvas.fill(0);
            })
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        let mut auras = self.auras.borrow_mut();
        auras.insert(aura.id.clone(), AuraInstance { surface });

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
