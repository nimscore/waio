use crate::infrastructure::lua;
use crate::infrastructure::wayland::{AuraSurface, WlState};
use crate::usecase::render::{RenderError, Renderer};
use slint::platform::software_renderer::{LineBufferProvider, SoftwareRenderer};
use slint_interpreter::{Compiler, ComponentInstance};
use smithay_client_toolkit::reexports::client::QueueHandle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::{error, info, warn};
use waio_shared::entity::Aura;

struct AuraInstance {
    instance: ComponentInstance,
    surface: AuraSurface,
}

struct FrameBuffer<'a> {
    frame_buffer: &'a mut [u8],
    width: u32,
}

impl<'a> LineBufferProvider for FrameBuffer<'a> {
    type TargetPixel = slint::Rgb8Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [Self::TargetPixel]),
    ) {
        let line_begin = line * (self.width as usize);
        let slice_start = (line_begin + range.start) * 4;
        let slice_end = (line_begin + range.end) * 4;
        if slice_end <= self.frame_buffer.len() {
            let pixel_slice = unsafe {
                std::slice::from_raw_parts_mut(
                    self.frame_buffer[slice_start..slice_end].as_mut_ptr() as *mut slint::Rgb8Pixel,
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
        }
    }

    fn compile(&self, code: &str) -> Result<slint_interpreter::ComponentDefinition, RenderError> {
        let compiler = Compiler::default();
        let result = spin_on::spin_on(
            compiler.build_from_source(code.to_string(), std::path::PathBuf::from("virtual.slint")),
        );

        for diag in result.diagnostics() {
            match diag.level() {
                slint_interpreter::DiagnosticLevel::Error => {
                    error!("Slint: {}", diag);
                }
                slint_interpreter::DiagnosticLevel::Warning => {
                    warn!("Slint: {}", diag);
                }
                _ => {
                    info!("Slint: {}", diag);
                }
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

    pub fn create_surface(
        &self,
        aura: &Aura,
        wl_state: &mut WlState,
    ) -> Result<AuraSurface, RenderError> {
        AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            &aura.config,
        )
        .map_err(|e| RenderError::WaylandError(e.to_string()))
    }

    pub fn draw_aura(&self, aura: &Aura, surface: &mut AuraSurface) -> Result<(), RenderError> {
        let definition = self.compile(&aura.slint_code)?;
        let instance = definition
            .create()
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        if let Some(ref code) = aura.lua_code {
            let lua = self.lua_state.borrow();
            let _ = lua.load(code).exec();
        }

        let renderer = SoftwareRenderer::default();

        surface
            .draw(|canvas, w, _h| {
                let frame_buffer = FrameBuffer {
                    frame_buffer: canvas,
                    width: w,
                };
                renderer.render_by_line(frame_buffer);
            })
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        let mut auras = self.auras.borrow_mut();
        auras.insert(
            aura.id.clone(),
            AuraInstance {
                instance,
                surface: unsafe { std::ptr::read(surface as *const _) },
            },
        );

        info!("Aura '{}' rendered successfully", aura.name);
        Ok(())
    }

    pub fn render_aura_with_state(
        &self,
        aura: &Aura,
        wl_state: &mut WlState,
    ) -> Result<(), RenderError> {
        info!("Rendering aura: {}", aura.name);

        let definition = self.compile(&aura.slint_code)?;
        let instance = definition
            .create()
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        if let Some(ref code) = aura.lua_code {
            let lua = self.lua_state.borrow();
            let _ = lua.load(code).exec();
        }

        let mut surface = AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            &aura.config,
        )
        .map_err(|e| RenderError::WaylandError(e.to_string()))?;

        let renderer = SoftwareRenderer::default();

        surface
            .draw(|canvas, w, _h| {
                let frame_buffer = FrameBuffer {
                    frame_buffer: canvas,
                    width: w,
                };
                renderer.render_by_line(frame_buffer);
            })
            .map_err(|e| RenderError::RenderFailed(e.to_string()))?;

        let mut auras = self.auras.borrow_mut();
        auras.insert(aura.id.clone(), AuraInstance { instance, surface });

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

    fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<(), RenderError> {
        self.remove_aura(aura_id)?;
        let aura = Aura {
            id: aura_id.to_string(),
            name: "Updated".into(),
            aura_type: waio_shared::entity::AuraType::Bar,
            slint_code: slint_code.to_string(),
            lua_code: None,
            config: Default::default(),
        };
        Err(RenderError::RenderFailed(format!(
            "Need wl_state to render: {}",
            aura.id
        )))
    }

    fn shutdown(&self) -> Result<(), RenderError> {
        let mut auras = self.auras.borrow_mut();
        auras.clear();
        Ok(())
    }
}
