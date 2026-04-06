use regex::Regex;
use slint::platform::software_renderer::{
    MinimalSoftwareWindow, RepaintBufferType, Rgb565Pixel, SoftwareRenderer,
};
use slint::PhysicalSize;
use slint_interpreter::{Compiler, ComponentInstance, Value};
use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::seat::pointer::PointerEventKind;
use smithay_client_toolkit::shell::wlr_layer::{LayerSurface, LayerSurfaceConfigure};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};
use tracing::{error, info, warn};

use crate::error::{Result, WaioError};
use crate::infrastructure::lua;
use crate::infrastructure::lua::input::InputRegistry;
use crate::infrastructure::lua::rate_limiter::RateLimiter;
use crate::infrastructure::lua::timer::{TimerFire, TimerRegistry};
use crate::infrastructure::slint::aura_handle::{new_command_queue, CommandQueue};
use crate::infrastructure::wayland::PointerState;
use crate::infrastructure::wayland::{AuraSurface, WlState};
#[allow(unused_imports)]
use crate::usecase::render::Renderer;
use smithay_client_toolkit::reexports::calloop::channel;
use smithay_client_toolkit::reexports::calloop::channel::Channel;
use waio_shared::entity::Aura;

/// A single renderable layer (sub-window) within an aura.
/// Each layer is a separate Slint component rendered to its own buffer,
/// then composited into the final frame buffer.
struct SubWindow {
    /// Name of the exported Slint component (e.g. "Background", "TimeLayer").
    name: String,
    /// Slint's software rendering window for this layer.
    window: Rc<MinimalSoftwareWindow>,
    /// The instantiated Slint component.
    component: ComponentInstance,
    /// Position in the final composite buffer.
    x: u32,
    y: u32,
    /// Size of this layer.
    width: u32,
    height: u32,
    /// Pixel buffer for this layer.
    pixels: Vec<Rgb565Pixel>,
    /// Whether this layer needs re-rendering.
    needs_render: bool,
    /// Whether this is a static layer (rendered once, never updated).
    #[allow(dead_code)]
    is_static: bool,
}

/// Per-aura Slint rendering state.
///
/// If the aura has sub-component layers, `sub_windows` contains one entry per layer.
/// Otherwise, falls back to single-component mode with `main_window`.
struct AuraRenderState {
    /// Sub-windows for per-layer rendering. Empty = single-component mode.
    sub_windows: Vec<SubWindow>,
    /// Main window for single-component mode (fallback).
    main_window: Option<Rc<MinimalSoftwareWindow>>,
    /// Main component for single-component mode.
    main_component: Option<ComponentInstance>,
    /// Pixel buffer for single-component mode.
    main_pixels: Option<Rc<RefCell<Vec<Rgb565Pixel>>>>,
    /// Total aura size (from config).
    width: u32,
    height: u32,
    /// Whether this aura needs rendering.
    needs_render: bool,
}

/// Central renderer managing all aura Slint components and Wayland surfaces.
///
/// # Memory Safety
/// All `Rc<RefCell<>>` fields are owned by this struct and have no back-references.
/// `WlState` holds a shared `Rc<SlintRenderer>` but does not own any of the inner
/// `Rc<RefCell<>>` collections — it only borrows them through method calls.
/// No reference cycles exist.
pub struct SlintRenderer {
    /// Per-aura Slint render state. Owned, no back-references.
    render_states: Rc<RefCell<HashMap<String, AuraRenderState>>>,
    /// Per-aura Wayland surface state. Owned, no back-references.
    surfaces: Rc<RefCell<HashMap<String, AuraSurface>>>,
    /// Cancellable Lua timers.
    timer_registry: TimerRegistry,
    /// Shared Lua state. Owned.
    lua_state: Rc<RefCell<mlua::Lua>>,
    /// Shared rate limiter for Lua module calls.
    rate_limiter: RateLimiter,
    /// Command queue for Lua → Slint property updates.
    command_queue: CommandQueue,
    /// Wayland resources needed for surface creation.
    compositor: CompositorState,
    qh: QueueHandle<WlState>,
    /// Shared pointer state from WlState, used to dispatch pointer events to Slint windows.
    pointer_state: Arc<Mutex<PointerState>>,
    /// Shared input callbacks registry, used to trigger Lua callbacks on pointer events.
    input_registry: InputRegistry,
}

impl SlintRenderer {
    pub fn new(
        compositor: CompositorState,
        qh: QueueHandle<WlState>,
        pointer_state: Arc<Mutex<PointerState>>,
    ) -> (Self, Channel<TimerFire>) {
        let (timer_tx, timer_rx) = channel::channel::<TimerFire>();
        let timer_registry = TimerRegistry::new(timer_tx);
        let lua = lua::create_sandboxed_lua();
        let input_registry = InputRegistry::new();
        let rate_limiter = RateLimiter::new();

        let self_ = Self {
            render_states: Rc::new(RefCell::new(HashMap::new())),
            surfaces: Rc::new(RefCell::new(HashMap::new())),
            timer_registry,
            lua_state: Rc::new(RefCell::new(lua)),
            rate_limiter,
            command_queue: new_command_queue(),
            compositor,
            qh,
            pointer_state,
            input_registry,
        };
        (self_, timer_rx)
    }

    // ─── Aura lifecycle ───

    /// Load an aura: compile Slint, create SubWindows, create Wayland surface, run Lua.
    pub fn load_aura(&self, aura: &Aura, external_id: &str, wl_state: &mut WlState) -> Result<()> {
        info!("Loading aura: {} (id={})", aura.name, external_id);

        // Parse sub-components from Slint code.
        let component_names = extract_component_names(&aura.slint_code);
        let has_layers = !aura.layers.is_empty() && !component_names.is_empty();

        if has_layers {
            info!(
                "Loading aura with {} sub-components: {:?}",
                component_names.len(),
                component_names
            );
            self.load_aura_with_layers(aura, external_id, &component_names)?;
        } else {
            info!("Loading aura in single-component mode");
            self.load_aura_single(aura, external_id)?;
        }

        // Create the Wayland layer surface.
        // Resolve output: explicit name from config → auto-detect (default output).
        let output_name = aura.config.output.as_deref();
        let output = wl_state.get_output(output_name);
        let output_label = if let Some(name) = output_name {
            if output.is_some() {
                format!("explicit: {}", name)
            } else {
                format!("explicit '{}' not found, using auto-detect", name)
            }
        } else {
            "auto-detect (default)".to_string()
        };
        info!("Output for aura '{}': {}", external_id, output_label);

        let surface = AuraSurface::new(
            &self.compositor,
            &wl_state.layer_shell,
            &wl_state.shm,
            &self.qh,
            external_id.to_string(),
            &aura.config,
            output,
        )
        .map_err(|e| WaioError::Wayland(anyhow::anyhow!("{}", e)))?;

        {
            let mut surfaces = self.surfaces.borrow_mut();
            surfaces.insert(external_id.to_string(), surface);
        }

        // Run Lua code in a restricted sandbox environment.
        if let Some(ref lua_code) = aura.lua_code {
            let lua = self.lua_state.borrow();

            // Create restricted environment for this aura.
            let env = lua::sandbox::create_restricted_env(&lua)
                .map_err(|e| WaioError::Lua(mlua::Error::external(e)))?;

            // Register slint bridge (sets in both globals and restricted env).
            lua::slint_bridge::register_slint_in_env(
                &lua,
                &env,
                external_id.to_string(),
                self.command_queue.clone(),
            )
            .map_err(|e| WaioError::Lua(mlua::Error::external(e)))?;

            // Register waio.timer — must be in BOTH globals (for timer callbacks)
            // and restricted env (for initial Lua code execution).
            let timer_module = lua::timer::create_module(
                &lua,
                self.timer_registry.clone(),
                external_id.to_string(),
            )?;

            // Add timer to globals waio table.
            let waio_globals: mlua::Table = if let Ok(t) = lua.globals().get("waio") {
                t
            } else {
                let t = lua.create_table()?;
                lua.globals().set("waio", &t)?;
                t
            };
            waio_globals.set("timer", timer_module)?;

            // Copy waio (now with timer) to restricted env.
            env.set("waio", &waio_globals)?;

            // Register waio.fs if the aura has fs_read permission.
            if aura.permissions.iter().any(|p| p == "fs_read") {
                let fs_access = lua::fs::FsAccess::new(vec![
                    std::env::current_dir().unwrap_or_default(),
                    dirs::config_dir().unwrap_or_default().join("waio/widgets"),
                    // Allow reading sysfs/procfs for system monitoring widgets.
                    std::path::PathBuf::from("/proc"),
                    std::path::PathBuf::from("/sys"),
                ]);
                let fs_module =
                    lua::fs::create_module(&lua, fs_access, Some(self.rate_limiter.clone()))?;
                waio_globals.set("fs", fs_module)?;
                info!("Registered waio.fs for aura: {}", external_id);
            }

            // Register waio.http if the aura has http permission.
            if aura.permissions.iter().any(|p| p == "http") {
                let http_access = lua::http::HttpAccess::new(vec![]);
                let http_module =
                    lua::http::create_module(&lua, http_access, Some(self.rate_limiter.clone()))?;
                waio_globals.set("http", http_module)?;
                info!("Registered waio.http for aura: {}", external_id);
            }

            // Register waio.notify, waio.backlight, waio.power, waio.audio if the aura has system permission.
            if aura.permissions.iter().any(|p| p == "system") {
                let notify_module =
                    lua::notify::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("notify", notify_module)?;
                info!("Registered waio.notify for aura: {}", external_id);

                let backlight_module =
                    lua::backlight::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("backlight", backlight_module)?;
                info!("Registered waio.backlight for aura: {}", external_id);

                let power_module =
                    lua::power::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("power", power_module)?;
                info!("Registered waio.power for aura: {}", external_id);

                let audio_module =
                    lua::audio::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("audio", audio_module)?;
                info!("Registered waio.audio for aura: {}", external_id);

                let bluetooth_module =
                    lua::bluetooth::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("bluetooth", bluetooth_module)?;
                info!("Registered waio.bluetooth for aura: {}", external_id);
            }

            // Register waio.input if the aura has input permission.
            if aura.permissions.iter().any(|p| p == "input") {
                let input_module = lua::input::create_module(
                    &lua,
                    self.input_registry.clone(),
                    external_id.to_string(),
                )?;
                waio_globals.set("input", input_module)?;
                info!("Registered waio.input for aura: {}", external_id);
            }

            // Register waio.exec if the aura has exec permission.
            if aura.permissions.iter().any(|p| p == "exec") {
                let exec_access = lua::exec::ExecAccess::new(aura.exec_commands.clone());
                let exec_module =
                    lua::exec::create_module(&lua, exec_access, Some(self.rate_limiter.clone()))?;
                waio_globals.set("exec", exec_module)?;
                if aura.exec_commands.is_empty() {
                    warn!(
                        "Registered waio.exec with no command whitelist for aura: {} — all commands allowed",
                        external_id
                    );
                } else {
                    info!(
                        "Registered waio.exec for aura: {} with {} allowed commands",
                        external_id,
                        aura.exec_commands.len()
                    );
                }
            }

            // Register waio.wifi if the aura has network permission.
            if aura.permissions.iter().any(|p| p == "network") {
                let wifi_module = lua::wifi::create_module(&lua, Some(self.rate_limiter.clone()))?;
                waio_globals.set("wifi", wifi_module)?;
                info!("Registered waio.wifi for aura: {}", external_id);
            }

            // Execute in sandboxed environment.
            lua.load(lua_code)
                .set_name(format!("aura:{}", external_id))
                .set_environment(env)
                .exec()
                .map_err(|e| WaioError::Lua(mlua::Error::external(e)))?;
            info!("Lua code executed for aura: {}", aura.id);
        }

        info!(
            "Aura '{}' loaded, waiting for compositor configure",
            aura.name
        );
        Ok(())
    }

    /// Load aura with sub-component layers.
    fn load_aura_with_layers(
        &self,
        aura: &Aura,
        external_id: &str,
        _component_names: &[String],
    ) -> Result<()> {
        // Compile the full Slint code ONCE.
        let compiler = Compiler::default();
        let result = spin_on::spin_on(compiler.build_from_source(
            aura.slint_code.clone(),
            std::path::PathBuf::from("virtual.slint"),
        ));

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

        let mut sub_windows = Vec::new();

        for layer in &aura.layers {
            // Get the component definition from the already-compiled result.
            let comp_def = result.component(&layer.name).ok_or_else(|| {
                WaioError::SlintCompilation(format!("Component '{}' not found", layer.name))
            })?;

            info!("Using component: {} ({}x{})", layer.name, layer.w, layer.h);

            let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
            window.set_size(PhysicalSize::new(layer.w, layer.h));

            let component = comp_def
                .create_with_existing_window(&window)
                .map_err(|e| WaioError::SlintCompilation(format!("{}: {}", layer.name, e)))?;

            sub_windows.push(SubWindow {
                name: layer.name.clone(),
                window,
                component,
                x: layer.x,
                y: layer.y,
                width: layer.w,
                height: layer.h,
                pixels: vec![Rgb565Pixel::default(); (layer.w * layer.h) as usize],
                needs_render: true,
                is_static: !layer.dynamic,
            });
        }

        {
            let mut states = self.render_states.borrow_mut();
            states.insert(
                external_id.to_string(),
                AuraRenderState {
                    sub_windows,
                    main_window: None,
                    main_component: None,
                    main_pixels: None,
                    width: aura.config.size.width,
                    height: aura.config.size.height,
                    needs_render: true,
                },
            );
        }

        Ok(())
    }

    /// Load aura in single-component mode (fallback).
    fn load_aura_single(&self, aura: &Aura, external_id: &str) -> Result<()> {
        let definition = self.compile_slint(&aura.slint_code)?;
        info!(
            "Slint component compiled: {}, id={}",
            definition.name(),
            external_id
        );

        let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
        window.set_size(PhysicalSize::new(
            aura.config.size.width,
            aura.config.size.height,
        ));

        let component = definition
            .create_with_existing_window(&window)
            .map_err(|e| WaioError::SlintCompilation(format!("create component: {}", e)))?;

        let pixel_count = (aura.config.size.width * aura.config.size.height) as usize;
        let pixels = Rc::new(RefCell::new(vec![Rgb565Pixel::default(); pixel_count]));

        {
            let mut states = self.render_states.borrow_mut();
            states.insert(
                external_id.to_string(),
                AuraRenderState {
                    sub_windows: Vec::new(),
                    main_window: Some(window),
                    main_component: Some(component),
                    main_pixels: Some(pixels),
                    width: aura.config.size.width,
                    height: aura.config.size.height,
                    needs_render: true,
                },
            );
        }

        Ok(())
    }

    /// Called when the compositor sends a configure event for a layer surface.
    pub fn on_surface_configured(
        &self,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        _serial: u32,
    ) -> Result<()> {
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

        let is_first = {
            let mut surfaces = self.surfaces.borrow_mut();
            if let Some(surface) = surfaces.get_mut(&id) {
                surface.on_configure(configure)
            } else {
                return Err(WaioError::AuraNotFound(id));
            }
        };

        if is_first {
            self.render_first_frame_for_aura(&id)
        } else {
            Ok(())
        }
    }

    /// Called when a frame callback is received.
    pub fn on_frame_complete(&self, surface: &WlSurface) -> Result<()> {
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

    /// Called when a layer surface is closed.
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
    pub fn process_commands(&self) -> Result<()> {
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

        // Apply property updates.
        // Property name format: "LayerName.property" or just "property" (single-component mode).
        let mut modified_auras: Vec<String> = Vec::new();

        {
            let render_states = self.render_states.borrow();
            for cmd in &commands {
                if !render_states.contains_key(&cmd.aura_id) {
                    continue;
                }

                let state = render_states.get(&cmd.aura_id).unwrap();

                // Try to parse "LayerName.property" format.
                if let Some((layer_name, prop_name)) = cmd.property.split_once('.') {
                    // Sub-component mode: find the matching SubWindow.
                    for sw in &state.sub_windows {
                        if sw.name == layer_name {
                            match sw.component.set_property(
                                prop_name,
                                Value::String(slint::SharedString::from(cmd.value.clone())),
                            ) {
                                Ok(_) => modified_auras.push(cmd.aura_id.clone()),
                                Err(e) => warn!(
                                    "Failed to set property {}.{}: {}",
                                    layer_name, prop_name, e
                                ),
                            }
                            break;
                        }
                    }
                } else {
                    // Single-component mode.
                    if let Some(ref component) = state.main_component {
                        match component.set_property(
                            &cmd.property,
                            Value::String(slint::SharedString::from(cmd.value.clone())),
                        ) {
                            Ok(_) => modified_auras.push(cmd.aura_id.clone()),
                            Err(e) => warn!("Failed to set property {}: {}", cmd.property, e),
                        }
                    }
                }
            }
        }

        // Mark auras as needing render.
        {
            let mut render_states = self.render_states.borrow_mut();
            for cmd in &commands {
                if let Some(state) = render_states.get_mut(&cmd.aura_id) {
                    state.needs_render = true;
                    // Mark matching sub-windows.
                    if let Some((layer_name, _)) = cmd.property.split_once('.') {
                        for sw in &mut state.sub_windows {
                            if sw.name == layer_name {
                                sw.needs_render = true;
                            }
                        }
                    }
                }
            }
        }

        // Mark Wayland surfaces as dirty.
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

    /// Process a single timer fire event — calls the Lua callback in the main thread (safe).
    /// Called from the calloop event loop channel callback.
    pub fn process_single_timer_fire(&self, fire: TimerFire) {
        self.timer_registry.process_single_fire(fire);
    }

    /// Redraw all dirty surfaces.
    pub fn redraw_all(&self) -> Result<()> {
        let dirty_ids = {
            let render_states = self.render_states.borrow();
            let surfaces = self.surfaces.borrow();
            render_states
                .keys()
                .filter(|id| {
                    render_states.get(*id).is_some_and(|s| s.needs_render)
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

    // ─── Pointer event dispatch ───

    /// Dispatch pending pointer events from `PointerState` to the appropriate Slint windows.
    ///
    /// For each pending event, determines which aura surface contains the pointer coordinates,
    /// then forwards the event as a Slint `WindowEvent`:
    /// - **Multi-layer auras**: performs AABB hit-testing against sub-windows; topmost layer wins.
    /// - **Single-component auras**: forwards directly to `main_window`.
    ///
    /// Coordinates are in logical pixels (scale_factor = 1.0 assumed).
    pub fn dispatch_pointer_events(&self) {
        // Take pending events from the shared pointer state.
        let events = {
            let mut ps = match self.pointer_state.lock() {
                Ok(guard) => guard,
                Err(e) => {
                    warn!("Failed to lock pointer state: {}", e);
                    return;
                }
            };
            std::mem::take(&mut ps.events)
        };

        if events.is_empty() {
            return;
        }

        let render_states = self.render_states.borrow();
        let surfaces = self.surfaces.borrow();

        for event in &events {
            let (px, py) = event.position;
            let logical_x = px as f32;
            let logical_y = py as f32;
            let logical_pos = slint::LogicalPosition::new(logical_x, logical_y);

            // Find the aura whose surface matches the event's surface.
            let aura_id = match surfaces
                .iter()
                .find(|(_, s)| s.surface().is_some_and(|ws| *ws == event.surface))
            {
                Some((id, _)) => id.clone(),
                None => continue, // Event for unknown surface.
            };

            let Some(state) = render_states.get(&aura_id) else {
                continue;
            };

            // Determine which window to dispatch to.
            let target_window = if !state.sub_windows.is_empty() {
                // Multi-layer: AABB hit-test, topmost layer wins.
                find_topmost_subwindow(&state.sub_windows, logical_x, logical_y)
            } else {
                // Single-component mode.
                state.main_window.clone()
            };

            let Some(window) = target_window else {
                continue;
            };

            // Convert the Wayland pointer event kind to a Slint WindowEvent.
            let slint_event = match &event.kind {
                PointerEventKind::Motion { .. } => {
                    Some(slint::platform::WindowEvent::PointerMoved {
                        position: logical_pos,
                    })
                }
                PointerEventKind::Press { button, .. } => {
                    Some(slint::platform::WindowEvent::PointerPressed {
                        position: logical_pos,
                        button: wayland_button_to_slint(*button),
                    })
                }
                PointerEventKind::Release { button, .. } => {
                    Some(slint::platform::WindowEvent::PointerReleased {
                        position: logical_pos,
                        button: wayland_button_to_slint(*button),
                    })
                }
                PointerEventKind::Axis {
                    horizontal,
                    vertical,
                    ..
                } => Some(slint::platform::WindowEvent::PointerScrolled {
                    position: logical_pos,
                    delta_x: horizontal.absolute as f32,
                    delta_y: vertical.absolute as f32,
                }),
                PointerEventKind::Enter { .. } | PointerEventKind::Leave { .. } => {
                    // Enter/Leave are handled by surface tracking; no Slint event needed.
                    None
                }
            };

            if let Some(evt) = slint_event {
                if let Err(e) = window.try_dispatch_event(evt) {
                    warn!(
                        "Failed to dispatch pointer event to aura '{}': {}",
                        aura_id, e
                    );
                }
            }

            // Trigger Lua input callbacks.
            match &event.kind {
                PointerEventKind::Press { button, .. } => {
                    // Use the topmost sub-window name as component_name.
                    let component_name = if !state.sub_windows.is_empty() {
                        find_topmost_subwindow_name(&state.sub_windows, logical_x, logical_y)
                            .unwrap_or_else(|| "unknown".to_string())
                    } else {
                        "main".to_string()
                    };
                    self.input_registry.trigger_on_click(
                        &aura_id,
                        *button,
                        logical_x,
                        logical_y,
                        &component_name,
                    );
                }
                PointerEventKind::Axis {
                    horizontal,
                    vertical,
                    ..
                } => {
                    self.input_registry.trigger_on_scroll(
                        &aura_id,
                        horizontal.absolute as f32,
                        vertical.absolute as f32,
                        logical_x,
                        logical_y,
                    );
                }
                PointerEventKind::Enter { .. } => {
                    self.input_registry
                        .trigger_on_hover(&aura_id, logical_x, logical_y, true);
                }
                PointerEventKind::Leave { .. } => {
                    self.input_registry
                        .trigger_on_hover(&aura_id, logical_x, logical_y, false);
                }
                _ => {}
            }
        }
    }

    /// Remove an aura and all its resources.
    pub fn remove_aura(&self, aura_id: &str) -> Result<()> {
        self.timer_registry.cancel_all_for_aura(aura_id);
        self.input_registry.clear_aura(aura_id);

        // Clear property store for this aura.
        crate::infrastructure::lua::slint_bridge::clear_property_store(aura_id);

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

    /// Render the first frame for an aura.
    fn render_first_frame_for_aura(&self, aura_id: &str) -> Result<()> {
        self.render_aura_to_canvas(aura_id, true)
    }

    /// Render a subsequent frame for an aura.
    fn render_frame_for_aura(&self, aura_id: &str) -> Result<()> {
        let needs_render = {
            let render_states = self.render_states.borrow();
            render_states.get(aura_id).is_some_and(|s| s.needs_render)
        };

        if !needs_render {
            return Ok(());
        }

        self.render_aura_to_canvas(aura_id, false)
    }

    /// Render an aura to the Wayland canvas.
    fn render_aura_to_canvas(&self, aura_id: &str, is_first_frame: bool) -> Result<()> {
        {
            let render_states = self.render_states.borrow();
            let state = render_states
                .get(aura_id)
                .ok_or_else(|| WaioError::AuraNotFound(aura_id.to_string()))?;

            if !state.sub_windows.is_empty() {
                // Sub-component mode: render each layer, composite, send to Wayland.
                self.render_composite_and_send(aura_id, state, is_first_frame)?;
            } else if let (Some(window), Some(component), Some(pixels)) = (
                &state.main_window,
                &state.main_component,
                &state.main_pixels,
            ) {
                // Single-component mode.
                render_single_to_canvas(window, component, pixels, state.width, state.height)?;
            }
        }

        // Clear needs_render flag.
        {
            let mut render_states = self.render_states.borrow_mut();
            if let Some(state) = render_states.get_mut(aura_id) {
                state.needs_render = false;
            }
        }

        Ok(())
    }

    /// Render each sub-window layer, composite into a single buffer, and send to Wayland.
    fn render_composite_and_send(
        &self,
        aura_id: &str,
        state: &AuraRenderState,
        _is_first_frame: bool,
    ) -> Result<()> {
        let aura_w = state.width as usize;
        let aura_h = state.height as usize;
        let mut final_pixels: Vec<Rgb565Pixel> = vec![Rgb565Pixel::default(); aura_w * aura_h];

        for sw in &state.sub_windows {
            // Render this sub-window.
            sw.window.request_redraw();
            let mut pixels = sw.pixels.clone();
            sw.window.draw_if_needed(|renderer: &SoftwareRenderer| {
                renderer.render(&mut pixels, sw.width as usize);
            });

            // Blit into final buffer at the layer's position.
            blit_layer(
                &mut final_pixels,
                &pixels,
                sw.x as usize,
                sw.y as usize,
                sw.width as usize,
                sw.height as usize,
                aura_w,
            );
        }

        // Convert RGB565 → ARGB8888.
        let mut canvas: Vec<u8> = vec![0; aura_w * aura_h * 4];
        for (i, pixel) in final_pixels.iter().enumerate() {
            let r = ((pixel.0 >> 11) & 0x1F) as u32;
            let g = ((pixel.0 >> 5) & 0x3F) as u32;
            let b = (pixel.0 & 0x1F) as u32;
            let idx = i * 4;
            if idx + 3 < canvas.len() {
                canvas[idx] = ((b << 3) | (b >> 2)) as u8; // B
                canvas[idx + 1] = ((g << 2) | (g >> 4)) as u8; // G
                canvas[idx + 2] = ((r << 3) | (r >> 2)) as u8; // R
                canvas[idx + 3] = 0xFF; // A
            }
        }

        // Send the composited canvas to the Wayland surface.
        let mut surfaces = self.surfaces.borrow_mut();
        if let Some(surface) = surfaces.get_mut(aura_id) {
            surface
                .draw_precomposited(&canvas)
                .map_err(|e| WaioError::SlintRender(format!("draw_precomposited: {}", e)))?;
        }

        Ok(())
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
                WaioError::SlintCompilation(format!("No component found. Available: {:?}", names))
            })?;

        Ok(definition)
    }
}

// ─── Pointer dispatch helpers ───

/// Find the topmost sub-window that contains the given logical (x, y) coordinates.
/// Layers are checked in order (last = topmost), so the last matching layer wins.
fn find_topmost_subwindow(
    sub_windows: &[SubWindow],
    x: f32,
    y: f32,
) -> Option<Rc<MinimalSoftwareWindow>> {
    // Iterate in reverse order so the topmost layer is checked first.
    sub_windows.iter().rev().find_map(|sw| {
        let left = sw.x as f32;
        let top = sw.y as f32;
        let right = (sw.x + sw.width) as f32;
        let bottom = (sw.y + sw.height) as f32;

        if x >= left && x < right && y >= top && y < bottom {
            Some(sw.window.clone())
        } else {
            None
        }
    })
}

/// Map Wayland button codes to Slint `PointerEventButton`.
///
/// Wayland button codes (from linux/input-event-codes.h):
/// - BTN_LEFT    = 0x110 (272)
/// - BTN_RIGHT   = 0x111 (273)
/// - BTN_MIDDLE  = 0x112 (274)
/// - BTN_SIDE    = 0x113 (275)  → Back
/// - BTN_EXTRA   = 0x114 (276)  → Forward
fn wayland_button_to_slint(button: u32) -> slint::platform::PointerEventButton {
    match button {
        0x110 => slint::platform::PointerEventButton::Left,
        0x111 => slint::platform::PointerEventButton::Right,
        0x112 => slint::platform::PointerEventButton::Middle,
        0x113 => slint::platform::PointerEventButton::Back,
        0x114 => slint::platform::PointerEventButton::Forward,
        _ => slint::platform::PointerEventButton::Other,
    }
}

/// Find the name of the topmost sub-window that contains the given logical (x, y).
fn find_topmost_subwindow_name(sub_windows: &[SubWindow], x: f32, y: f32) -> Option<String> {
    sub_windows.iter().rev().find_map(|sw| {
        let left = sw.x as f32;
        let top = sw.y as f32;
        let right = (sw.x + sw.width) as f32;
        let bottom = (sw.y + sw.height) as f32;

        if x >= left && x < right && y >= top && y < bottom {
            Some(sw.name.clone())
        } else {
            None
        }
    })
}

/// Render a single-component aura to the canvas.
fn render_single_to_canvas(
    window: &MinimalSoftwareWindow,
    _component: &ComponentInstance,
    pixels: &Rc<RefCell<Vec<Rgb565Pixel>>>,
    width: u32,
    height: u32,
) -> Result<()> {
    let mut px = pixels.borrow_mut();
    let pixel_count = (width * height) as usize;

    if px.len() != pixel_count {
        px.resize(pixel_count, Rgb565Pixel::default());
    }

    window.request_redraw();
    window.draw_if_needed(|renderer: &SoftwareRenderer| {
        renderer.render(&mut px, width as usize);
    });

    // Note: conversion to ARGB8888 and sending to Wayland happens in surface.draw()
    // This function only updates the pixel buffer.

    Ok(())
}

/// Blit a layer into the final composite buffer.
/// Only copies non-black pixels (treats black as transparent).
fn blit_layer(
    final_buffer: &mut [Rgb565Pixel],
    layer_pixels: &[Rgb565Pixel],
    dst_x: usize,
    dst_y: usize,
    layer_w: usize,
    layer_h: usize,
    final_stride: usize,
) {
    for y in 0..layer_h {
        for x in 0..layer_w {
            let src_idx = y * layer_w + x;
            let dst_idx = (dst_y + y) * final_stride + (dst_x + x);
            if dst_idx < final_buffer.len()
                && src_idx < layer_pixels.len()
                && layer_pixels[src_idx].0 != 0
            // Skip black (transparent) pixels
            {
                final_buffer[dst_idx] = layer_pixels[src_idx];
            }
        }
    }
}

/// Pre-compiled regex for extracting component names from Slint source.
static COMPONENT_NAME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"export\s+component\s+(\w+)").unwrap());

/// Extract all `export component` names from Slint source code.
fn extract_component_names(code: &str) -> Vec<String> {
    COMPONENT_NAME_RE
        .captures_iter(code)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
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
        self.timer_registry.cancel_all();
        self.input_registry.clear_all();

        let mut surfaces = self.surfaces.borrow_mut();
        for (_, surface) in surfaces.iter_mut() {
            surface.close();
        }
        surfaces.clear();
        self.render_states.borrow_mut().clear();
        Ok(())
    }
}
