use slint::platform::software_renderer::{
    MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel, SoftwareRenderer,
};
use slint::PhysicalSize;
use slint_interpreter::{Compiler, ComponentInstance, Value};
use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::shell::wlr_layer::{LayerSurface, LayerSurfaceConfigure};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::{error, info, warn};

use crate::error::{Result, WaioError};
use crate::infrastructure::lua;
use crate::infrastructure::lua::timer::TimerRegistry;
use crate::infrastructure::slint::aura_handle::{new_command_queue, CommandQueue};
use crate::infrastructure::wayland::{AuraSurface, WlState};
#[allow(unused_imports)]
use crate::usecase::render::Renderer;
use waio_shared::entity::Aura;

/// Per-aura Slint rendering state.
///
/// Each aura gets its own `MinimalSoftwareWindow` and `ComponentInstance`.
/// This is critical: sharing a single window across multiple auras causes
/// size conflicts, property clobbering, and render corruption.
struct AuraRenderState {
    /// The compiled and instantiated Slint component.
    component: ComponentInstance,
    /// Slint's software rendering window — one per aura.
    window: Rc<MinimalSoftwareWindow>,
    /// Size in physical pixels (from aura config, may be overridden by compositor).
    width: u32,
    height: u32,
    /// Whether this aura needs redrawing on the next frame.
    dirty: bool,
    /// Persistent pixel buffer for Slint rendering.
    /// Wrapped in Rc for cheap cloning (all clones share the same Vec).
    pixels: Rc<RefCell<Vec<Rgb565Pixel>>>,
}

/// Central renderer managing all aura Slint components and Wayland surfaces.
///
/// Lifecycle:
/// 1. `load_aura()` — compile Slint, create window + surface, run Lua. Surface enters Pending state.
/// 2. `on_surface_configured()` — compositor confirmed sizes. Render first frame.
/// 3. `process_commands()` — apply Lua property updates, mark surfaces dirty.
/// 4. `redraw_all()` — render all dirty surfaces.
/// 5. `on_frame_complete()` — frame callback received, render next frame if dirty.
pub struct SlintRenderer {
    /// Per-aura Slint render state (component + window).
    render_states: Rc<RefCell<HashMap<String, AuraRenderState>>>,
    /// Per-aura Wayland surface state.
    surfaces: Rc<RefCell<HashMap<String, AuraSurface>>>,
    /// Cancellable Lua timers.
    timer_registry: TimerRegistry,
    /// Shared Lua state.
    lua_state: Rc<RefCell<mlua::Lua>>,
    /// Command queue for Lua → Slint property updates.
    command_queue: CommandQueue,
    /// Wayland resources needed for surface creation.
    compositor: CompositorState,
    qh: QueueHandle<WlState>,
}

impl SlintRenderer {
    pub fn new(compositor: CompositorState, qh: QueueHandle<WlState>) -> Self {
        let timer_registry = TimerRegistry::new();
        let lua = mlua::Lua::new();
        let _ = lua::register_all(&lua, timer_registry.clone());

        Self {
            render_states: Rc::new(RefCell::new(HashMap::new())),
            surfaces: Rc::new(RefCell::new(HashMap::new())),
            timer_registry,
            lua_state: Rc::new(RefCell::new(lua)),
            command_queue: new_command_queue(),
            compositor,
            qh,
        }
    }

    // ─── Aura lifecycle ───

    /// Load an aura: compile Slint, create Wayland surface (pending), run Lua.
    ///
    /// The `external_id` is the ID provided by the caller (CLI), used as the key
    /// in render_states and surfaces maps. This ensures load/unload use the same key.
    pub fn load_aura(
        &self,
        aura: &Aura,
        external_id: &str,
        wl_state: &mut WlState,
    ) -> Result<()> {
        info!("Loading aura: {} (id={})", aura.name, external_id);

        // 1. Compile Slint component from source.
        let definition = self.compile_slint(&aura.slint_code)?;
        info!(
            "Slint component compiled: {}, id={}",
            definition.name(),
            external_id
        );

        // 2. Create a dedicated MinimalSoftwareWindow for this aura.
        //    NewBuffer allocates a fresh buffer each frame, ensuring Slint draws
        //    the COMPLETE scene (all text, including unchanged characters).
        //    With ReusedBuffer, only changed characters are redrawn, leaving
        //    "ghost" artifacts of old characters when using transparent backgrounds.
        let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
        window.set_size(PhysicalSize::new(
            aura.config.size.width,
            aura.config.size.height,
        ));

        // 3. Instantiate the component with this window.
        let component = definition
            .create_with_existing_window(&window)
            .map_err(|e| WaioError::SlintCompilation(format!("create component: {}", e)))?;

        // 4. Pre-allocate the persistent pixel buffer.
        let pixel_count = (aura.config.size.width * aura.config.size.height) as usize;
        let pixels = Rc::new(RefCell::new(vec![Rgb565Pixel::default(); pixel_count]));

        // 5. Create the Wayland layer surface (Pending state, initial commit sent).
        let surface = AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            external_id.to_string(),
            &aura.config,
        )
        .map_err(|e| WaioError::Wayland(anyhow::anyhow!("{}", e)))?;

        // 6. Register both in our maps.
        {
            let mut states = self.render_states.borrow_mut();
            states.insert(
                external_id.to_string(),
                AuraRenderState {
                    component,
                    window,
                    width: aura.config.size.width,
                    height: aura.config.size.height,
                    dirty: false,
                    pixels,
                },
            );
        }
        {
            let mut surfaces = self.surfaces.borrow_mut();
            surfaces.insert(external_id.to_string(), surface);
        }

        // 7. Run Lua code with command queue bridge.
        if let Some(ref lua_code) = aura.lua_code {
            let lua = self.lua_state.borrow();
            lua::slint_bridge::register_with_queue(
                &lua,
                external_id.to_string(),
                self.command_queue.clone(),
            )
            .map_err(|e| WaioError::Lua(mlua::Error::external(e)))?;

            match lua.load(lua_code).exec() {
                Ok(_) => info!("Lua code executed for aura: {}", aura.id),
                Err(e) => error!("Lua error for aura {}: {}", aura.id, e),
            }
        }

        info!(
            "Aura '{}' loaded, waiting for compositor configure",
            aura.name
        );
        Ok(())
    }

    /// Called when the compositor sends a configure event for a layer surface.
    ///
    /// SCTK automatically calls `ack_configure(serial)` BEFORE invoking our handler,
    /// so we do NOT need to call it manually.
    ///
    /// This method:
    /// 1. Finds the aura by matching the layer surface
    /// 2. Calls surface.on_configure() to transition Pending → Configured
    /// 3. If first configure, triggers the first frame render
    pub fn on_surface_configured(
        &self,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) -> Result<()> {
        // Find the aura ID by matching the layer surface using PartialEq (compares Wayland object ID).
        let aura_id = {
            let surfaces = self.surfaces.borrow();
            surfaces
                .iter()
                .find(|(_, surface)| surface.layer_surface().is_some_and(|ls| *ls == *layer))
                .map(|(id, _)| id.clone())
        };

        let Some(id) = aura_id else {
            warn!(
                "Received configure for unknown layer surface (ptr={:p})",
                layer
            );
            return Ok(());
        };

        info!("Layer surface configured for aura: {}", id);

        // Transition the surface state: Pending → Configured (or update sizes).
        let is_first = {
            let mut surfaces = self.surfaces.borrow_mut();
            if let Some(surface) = surfaces.get_mut(&id) {
                surface.on_configure(configure)
            } else {
                return Err(WaioError::AuraNotFound(id));
            }
        };

        // If this is the first configure, render the initial frame.
        if is_first {
            self.render_first_frame_for_aura(&id)
        } else {
            Ok(())
        }
    }

    /// Called when a frame callback is received for a Wayland surface.
    ///
    /// If the surface is dirty, renders the next frame and requests another frame callback.
    pub fn on_frame_complete(&self, surface: &WlSurface) -> Result<()> {
        // Find the aura ID by matching the wl_surface using PartialEq.
        let aura_id = {
            let surfaces = self.surfaces.borrow();
            surfaces
                .iter()
                .find(|(_, s)| s.surface().is_some_and(|ws| *ws == *surface))
                .map(|(id, _)| id.clone())
        };

        let Some(id) = aura_id else {
            warn!("Received frame callback for unknown surface");
            return Ok(());
        };

        self.render_frame_for_aura(&id)
    }

    /// Called when a layer surface is closed by the compositor.
    pub fn on_surface_closed(&self, layer: &LayerSurface) {
        let aura_id = {
            let surfaces = self.surfaces.borrow();
            surfaces
                .iter()
                .find(|(_, surface)| surface.layer_surface().is_some_and(|ls| *ls == *layer))
                .map(|(id, _)| id.clone())
        };

        if let Some(id) = aura_id {
            info!("Removing closed aura: {}", id);
            let _ = self.remove_aura(&id);
        }
    }

    /// Process pending property updates from Lua command queue.
    ///
    /// Drains all commands, applies property updates to the corresponding Slint components,
    /// and marks affected surfaces as dirty.
    pub fn process_commands(&self) -> Result<()> {
        // Drain all pending commands.
        let commands = {
            let mut q = self
                .command_queue
                .lock()
                .map_err(|e| WaioError::SlintRender(format!("queue lock: {}", e)))?;
            std::mem::take(&mut *q)
        };

        if commands.is_empty() {
            return Ok(());
        }

        // Collect which auras were modified.
        // Silently skip commands for removed auras (Lua timers can't be stopped).
        let mut modified_auras: Vec<String> = Vec::new();

        {
            let render_states = self.render_states.borrow();
            for cmd in &commands {
                if !render_states.contains_key(&cmd.aura_id) {
                    continue; // Silently skip stale commands
                }
                if let Some(render_state) = render_states.get(&cmd.aura_id) {
                    match render_state.component.set_property(
                        &cmd.property,
                        Value::String(slint::SharedString::from(cmd.value.clone())),
                    ) {
                        Ok(_) => {
                            modified_auras.push(cmd.aura_id.clone());
                        }
                        Err(e) => {
                            warn!("Failed to set property {}: {}", cmd.property, e)
                        }
                    }
                }
            }
        }

        // Mark modified auras as dirty.
        {
            let mut render_states = self.render_states.borrow_mut();
            for id in &modified_auras {
                if let Some(state) = render_states.get_mut(id) {
                    state.dirty = true;
                }
            }
        }

        // Mark surfaces as dirty too (for the surface state machine).
        {
            let mut surfaces = self.surfaces.borrow_mut();
            for id in &modified_auras {
                if let Some(surface) = surfaces.get_mut(id) {
                    surface.mark_dirty();
                }
            }
        }

        Ok(())
    }

    /// Redraw all dirty surfaces.
    ///
    /// Iterates through all aura surfaces and renders those marked as dirty.
    pub fn redraw_all(&self) -> Result<()> {
        // Collect IDs of dirty, rendering auras first to avoid borrow conflicts.
        let dirty_ids = {
            let render_states = self.render_states.borrow();
            let surfaces = self.surfaces.borrow();
            render_states
                .keys()
                .filter(|id| {
                    render_states.get(*id).is_some_and(|s| s.dirty)
                        && surfaces.get(*id).is_some_and(|s| s.is_rendering())
                })
                .cloned()
                .collect::<Vec<_>>()
        };

        for id in dirty_ids {
            if let Err(e) = self.render_frame_for_aura(&id) {
                warn!("Failed to redraw aura {}: {}", id, e);
            }
        }

        Ok(())
    }

    /// Remove an aura and all its resources.
    pub fn remove_aura(&self, aura_id: &str) -> Result<()> {
        {
            let mut surfaces = self.surfaces.borrow_mut();
            if let Some(surface) = surfaces.get_mut(aura_id) {
                surface.close();
            }
            surfaces.remove(aura_id);
        }

        {
            let mut render_states = self.render_states.borrow_mut();
            render_states.remove(aura_id);
        }

        info!("Aura removed: {}", aura_id);
        Ok(())
    }

    // ─── Private helpers ───

    /// Render the first frame for an aura (triggered after configure).
    fn render_first_frame_for_aura(&self, aura_id: &str) -> Result<()> {
        let (window, width, height, pixels) = {
            let render_states = self.render_states.borrow();
            let render_state = render_states
                .get(aura_id)
                .ok_or_else(|| WaioError::AuraNotFound(aura_id.to_string()))?;

            (
                render_state.window.clone(),
                render_state.width,
                render_state.height,
                render_state.pixels.clone(),
            )
        };

        let mut surfaces = self.surfaces.borrow_mut();
        let surface = surfaces
            .get_mut(aura_id)
            .ok_or_else(|| WaioError::AuraNotFound(aura_id.to_string()))?;

        surface
            .render_first_frame(|canvas, canvas_w, canvas_h| {
                render_slint_to_canvas_with_persistent_buffer(
                    &window,
                    canvas,
                    canvas_w,
                    canvas_h,
                    width,
                    height,
                    &pixels,
                );
            })
            .map_err(|e| WaioError::SlintRender(format!("first frame: {}", e)))
    }

    /// Render a subsequent frame for an aura.
    fn render_frame_for_aura(&self, aura_id: &str) -> Result<()> {
        // Check if dirty first.
        {
            let render_states = self.render_states.borrow();
            let Some(render_state) = render_states.get(aura_id) else {
                return Err(WaioError::AuraNotFound(aura_id.to_string()));
            };
            if !render_state.dirty {
                return Ok(());
            }
        }

        let (window, width, height, pixels) = {
            let render_states = self.render_states.borrow();
            let render_state = render_states
                .get(aura_id)
                .ok_or_else(|| WaioError::AuraNotFound(aura_id.to_string()))?;

            (
                render_state.window.clone(),
                render_state.width,
                render_state.height,
                render_state.pixels.clone(),
            )
        };

        let mut surfaces = self.surfaces.borrow_mut();
        let surface = surfaces
            .get_mut(aura_id)
            .ok_or_else(|| WaioError::AuraNotFound(aura_id.to_string()))?;

        surface
            .render_frame(|canvas, canvas_w, canvas_h| {
                render_slint_to_canvas_with_persistent_buffer(
                    &window,
                    canvas,
                    canvas_w,
                    canvas_h,
                    width,
                    height,
                    &pixels,
                );
            })
            .map_err(|e| WaioError::SlintRender(format!("frame: {}", e)))
    }

    fn compile_slint(
        &self,
        code: &str,
    ) -> std::result::Result<slint_interpreter::ComponentDefinition, WaioError> {
        let compiler = Compiler::default();
        let result = spin_on::spin_on(
            compiler.build_from_source(code.to_string(), std::path::PathBuf::from("virtual.slint")),
        );

        for diag in result.diagnostics() {
            match diag.level() {
                slint_interpreter::DiagnosticLevel::Error => {
                    error!("Slint compilation error: {}", diag)
                }
                slint_interpreter::DiagnosticLevel::Warning => {
                    warn!("Slint compilation warning: {}", diag)
                }
                _ => info!("Slint compilation info: {}", diag),
            }
        }

        let definition = result
            .component("AuraBar")
            .or_else(|| result.components().next())
            .ok_or_else(|| {
                let names: Vec<_> = result.components().map(|c| c.name().to_string()).collect();
                WaioError::SlintCompilation(format!(
                    "No component found. Available: {:?}",
                    names
                ))
            })?;

        Ok(definition)
    }
}

/// Render a Slint component to an ARGB8888 canvas.
///
/// With RepaintBufferType::NewBuffer, Slint allocates a fresh buffer each frame
/// and draws the COMPLETE scene. No pixel restoration needed.
fn render_slint_to_canvas_with_persistent_buffer(
    window: &MinimalSoftwareWindow,
    canvas: &mut [u8],
    canvas_w: u32,
    canvas_h: u32,
    _src_w: u32,
    _src_h: u32,
    persistent_pixels: &Rc<RefCell<Vec<Rgb565Pixel>>>,
) {
    let pixel_count = (canvas_w * canvas_h) as usize;

    {
        let mut pixels = persistent_pixels.borrow_mut();

        // Resize buffer if needed.
        if pixels.len() != pixel_count {
            tracing::warn!(
                "Pixel buffer resized: {} -> {} (possible resize event)",
                pixels.len(),
                pixel_count
            );
            pixels.resize(pixel_count, Rgb565Pixel::default());
        }

        // Always request redraw — Slint draws the COMPLETE scene into a fresh buffer.
        window.request_redraw();

        // NewBuffer gives us a clean buffer every time, so Slint renders all text
        // including unchanged characters. No ghosting artifacts.
        window.draw_if_needed(|renderer: &SoftwareRenderer| {
            renderer.render(&mut *pixels, canvas_w as usize);
        });

        // Convert Rgb565 → Argb8888 and write to canvas.
        for (i, pixel) in pixels.iter().enumerate() {
            let r = ((pixel.0 >> 11) & 0x1F) as u32;
            let g = ((pixel.0 >> 5) & 0x3F) as u32;
            let b = (pixel.0 & 0x1F) as u32;

            let idx = i * 4;
            if idx + 3 < canvas.len() {
                canvas[idx + 0] = ((b << 3) | (b >> 2)) as u8; // B
                canvas[idx + 1] = ((g << 2) | (g >> 4)) as u8; // G
                canvas[idx + 2] = ((r << 3) | (r >> 2)) as u8; // R
                canvas[idx + 3] = 0xFF; // A
            }
        }
    }
}

impl crate::usecase::render::Renderer for SlintRenderer {
    fn init(&self) -> Result<()> {
        info!("SlintRenderer initialized");
        Ok(())
    }

    fn render_aura(&self, _aura: &Aura) -> Result<()> {
        Err(WaioError::SlintRender(
            "Use load_aura + on_surface_configured instead".into(),
        ))
    }

    fn remove_aura(&self, aura_id: &str) -> Result<()> {
        self.remove_aura(aura_id)
    }

    fn update_aura(&self, _aura_id: &str, _slint_code: &str) -> Result<()> {
        Err(WaioError::SlintRender("Not yet implemented".into()))
    }

    fn shutdown(&self) -> Result<()> {
        // Stop all timer threads.
        self.timer_registry.cancel_all();

        // Close all surfaces.
        let mut surfaces = self.surfaces.borrow_mut();
        for (_, surface) in surfaces.iter_mut() {
            surface.close();
        }
        surfaces.clear();
        self.render_states.borrow_mut().clear();
        Ok(())
    }
}
