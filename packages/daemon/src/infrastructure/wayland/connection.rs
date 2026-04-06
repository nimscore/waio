use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use smithay_client_toolkit::compositor::{CompositorHandler, CompositorState};
use smithay_client_toolkit::output::{OutputHandler, OutputState};
use smithay_client_toolkit::reexports::client::globals::registry_queue_init;
use smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput;
use smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::{Connection, EventQueue, QueueHandle};
use smithay_client_toolkit::registry::{ProvidesRegistryState, RegistryState};
use smithay_client_toolkit::seat::pointer::{PointerEvent, PointerHandler};
use smithay_client_toolkit::seat::{Capability, SeatHandler, SeatState};
use smithay_client_toolkit::shell::wlr_layer::{
    LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure,
};
use smithay_client_toolkit::shm::{Shm, ShmHandler};
use smithay_client_toolkit::{
    delegate_compositor, delegate_layer, delegate_output, delegate_pointer, delegate_registry,
    delegate_seat, delegate_shm,
};
use tracing::{debug, info, warn};

use crate::infrastructure::slint::SlintRenderer;

/// Track available outputs for multi-monitor support.
pub struct OutputTracker {
    /// Map of output name → WlOutput.
    outputs: HashMap<String, WlOutput>,
    /// Default output (first connected).
    default_output: Option<WlOutput>,
}

impl OutputTracker {
    pub fn new() -> Self {
        Self {
            outputs: HashMap::new(),
            default_output: None,
        }
    }

    #[allow(dead_code)]
    pub fn get_output(&self, name: Option<&str>) -> Option<&WlOutput> {
        if let Some(name) = name {
            // Try explicit name; fall back to default if not found.
            self.outputs.get(name).or(self.default_output.as_ref())
        } else {
            // No name specified — use default.
            self.default_output.as_ref()
        }
    }

    pub fn add_output(&mut self, name: String, output: WlOutput) {
        if self.default_output.is_none() {
            self.default_output = Some(output.clone());
        }
        self.outputs.insert(name, output);
    }

    pub fn remove_output(&mut self, name: &str) {
        self.outputs.remove(name);
        if self.default_output.is_some()
            && self
                .outputs
                .values()
                .all(|o| o != self.default_output.as_ref().unwrap())
        {
            self.default_output = self.outputs.values().next().cloned();
        }
    }
}

/// Shared pointer state that tracks cursor position, button states, and scroll events.
/// This is stored in an `Arc<Mutex<>>` so the renderer can read pointer events.
#[derive(Debug, Default)]
pub struct PointerState {
    /// Current pointer position in surface-local coordinates (x, y).
    pub position: (f64, f64),
    /// Currently pressed buttons.
    pub buttons: Vec<u32>,
    /// Pending scroll events (horizontal, vertical) accumulated since last read.
    pub scroll_delta: (f64, f64),
    /// The surface the pointer is currently over.
    pub focused_surface: Option<WlSurface>,
    /// Collected pointer events since last read (consumed by the renderer).
    pub events: Vec<PointerEvent>,
}

impl PointerState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any mouse button is currently pressed.
    #[allow(dead_code)]
    pub fn is_button_pressed(&self) -> bool {
        !self.buttons.is_empty()
    }

    /// Check if a specific button is pressed.
    #[allow(dead_code)]
    pub fn is_button_pressed_id(&self, button: u32) -> bool {
        self.buttons.contains(&button)
    }

    /// Consume and return pending scroll deltas.
    #[allow(dead_code)]
    pub fn take_scroll_delta(&mut self) -> (f64, f64) {
        std::mem::take(&mut self.scroll_delta)
    }

    /// Consume and return pending pointer events.
    #[allow(dead_code)]
    pub fn take_events(&mut self) -> Vec<PointerEvent> {
        std::mem::take(&mut self.events)
    }
}

pub struct WlState {
    pub registry_state: RegistryState,
    pub output_state: OutputState,
    pub compositor_state: CompositorState,
    pub layer_shell: LayerShell,
    pub shm: Shm,
    pub seat_state: SeatState,
    /// Optional reference to the renderer. Set after renderer is created.
    pub renderer: Option<Rc<SlintRenderer>>,
    /// Track available outputs for multi-monitor support.
    output_tracker: OutputTracker,
    /// Shared pointer state accessible by the renderer.
    pub pointer_state: Arc<Mutex<PointerState>>,
    /// Currently known seats (seat name → WlSeat).
    seats: HashMap<String, WlSeat>,
}

impl WlState {
    /// Get an output by name, or the default output if name is None.
    #[allow(dead_code)]
    pub fn get_output(&self, name: Option<&str>) -> Option<&WlOutput> {
        self.output_tracker.get_output(name)
    }
}

pub struct WaylandConnection {
    pub conn: Connection,
    pub qh: QueueHandle<WlState>,
}

impl WaylandConnection {
    pub fn connect() -> Result<(Self, EventQueue<WlState>, WlState)> {
        info!("Connecting to Wayland...");
        let conn = Connection::connect_to_env()?;
        let (globals, event_queue) = registry_queue_init::<WlState>(&conn)?;
        let qh = event_queue.handle();

        let compositor_state = CompositorState::bind(&globals, &qh)?;
        let output_state = OutputState::new(&globals, &qh);
        let layer_shell = LayerShell::bind(&globals, &qh)?;
        let shm = Shm::bind(&globals, &qh)?;
        let seat_state = SeatState::new(&globals, &qh);

        let pointer_state = Arc::new(Mutex::new(PointerState::new()));

        let wl_state = WlState {
            registry_state: RegistryState::new(&globals),
            output_state,
            compositor_state: compositor_state.clone(),
            layer_shell,
            shm,
            seat_state,
            renderer: None,
            output_tracker: OutputTracker::new(),
            pointer_state,
            seats: HashMap::new(),
        };

        info!("Connected to Wayland");

        let wayland_conn = Self { conn, qh };

        Ok((wayland_conn, event_queue, wl_state))
    }
}

impl CompositorHandler for WlState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _new_transform: smithay_client_toolkit::reexports::client::protocol::wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        surface: &WlSurface,
        time: u32,
    ) {
        debug!("Frame callback: surface time={}", time);

        if let Some(ref renderer) = self.renderer {
            if let Err(e) = renderer.on_frame_complete(surface) {
                tracing::warn!("Frame callback error: {}", e);
            }
        }
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &WlSurface,
        _output: &WlOutput,
    ) {
    }
}

impl OutputHandler for WlState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, output: WlOutput) {
        // Try to get output info for the name.
        let name = self
            .output_state
            .info(&output)
            .and_then(|info| info.name.clone())
            .unwrap_or_else(|| format!("output-{}", self.output_tracker.outputs.len()));

        info!("New output connected: {}", name);
        self.output_tracker.add_output(name, output);
    }

    fn update_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {}

    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, output: WlOutput) {
        if let Some(info) = self.output_state.info(&output) {
            if let Some(name) = &info.name {
                info!("Output disconnected: {}", name);
                self.output_tracker.remove_output(name);
            }
        }
    }
}

impl LayerShellHandler for WlState {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        info!("Layer surface closed");

        if let Some(ref renderer) = self.renderer {
            renderer.on_surface_closed(layer);
        }
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: LayerSurfaceConfigure,
        serial: u32,
    ) {
        info!(
            "Layer surface configure: {:?}x{:?}, serial={}",
            configure.new_size.0, configure.new_size.1, serial
        );

        // SCTK automatically calls `ack_configure(serial)` BEFORE our callback.
        // We do NOT need to call it manually.

        if let Some(ref renderer) = self.renderer {
            if let Err(e) = renderer.on_surface_configured(layer, configure, serial) {
                tracing::error!("Failed to render after configure: {}", e);
            }
        }
    }
}

impl ShmHandler for WlState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

// ─── SeatHandler ───

impl SeatHandler for WlState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, seat: WlSeat) {
        info!("New seat detected, waiting for capability announcement");
        // Store the seat — the actual pointer is created in `new_capability`.
        // We use the seat object itself as a key by converting to string.
        let seat_id = format!("seat-{}", self.seats.len());
        self.seats.insert(seat_id, seat);
    }

    fn new_capability(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        seat: WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Pointer {
            info!("Pointer capability available on seat, creating pointer");
            match self.seat_state.get_pointer(qh, &seat) {
                Ok(_pointer) => {
                    info!("Pointer created successfully");
                }
                Err(e) => {
                    warn!("Failed to create pointer: {}", e);
                }
            }
        }
    }

    fn remove_capability(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _seat: WlSeat,
        capability: Capability,
    ) {
        if capability == Capability::Pointer {
            info!("Pointer capability removed");
            // The pointer will be released automatically when dropped.
            // Clear pointer state since the pointer is no longer valid.
            if let Ok(mut ps) = self.pointer_state.lock() {
                ps.buttons.clear();
                ps.focused_surface = None;
            }
        }
    }

    fn remove_seat(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, seat: WlSeat) {
        info!("Seat removed");
        // Remove from our map — find by value comparison.
        self.seats.retain(|_, s| s != &seat);
    }
}

// ─── PointerHandler ───

impl PointerHandler for WlState {
    fn pointer_frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _pointer: &smithay_client_toolkit::reexports::client::protocol::wl_pointer::WlPointer,
        events: &[PointerEvent],
    ) {
        let mut ps = self
            .pointer_state
            .lock()
            .expect("PointerState mutex poisoned");

        for event in events {
            debug!(
                "Pointer event: position=({:?}), kind={:?}",
                event.position, event.kind
            );

            // Update current position on motion/enter.
            match &event.kind {
                smithay_client_toolkit::seat::pointer::PointerEventKind::Enter { .. } => {
                    ps.position = event.position;
                    ps.focused_surface = Some(event.surface.clone());
                }
                smithay_client_toolkit::seat::pointer::PointerEventKind::Leave { .. } => {
                    ps.focused_surface = None;
                }
                smithay_client_toolkit::seat::pointer::PointerEventKind::Motion { .. } => {
                    ps.position = event.position;
                }
                smithay_client_toolkit::seat::pointer::PointerEventKind::Press {
                    button, ..
                } => {
                    if !ps.buttons.contains(button) {
                        ps.buttons.push(*button);
                    }
                }
                smithay_client_toolkit::seat::pointer::PointerEventKind::Release {
                    button, ..
                } => {
                    ps.buttons.retain(|b| b != button);
                }
                smithay_client_toolkit::seat::pointer::PointerEventKind::Axis {
                    horizontal,
                    vertical,
                    ..
                } => {
                    ps.scroll_delta.0 += horizontal.absolute;
                    ps.scroll_delta.1 += vertical.absolute;
                }
            }

            // Store the event for the renderer to consume.
            ps.events.push(event.clone());
        }
    }
}

delegate_compositor!(WlState);
delegate_output!(WlState);
delegate_layer!(WlState);
delegate_registry!(WlState);
delegate_shm!(WlState);
delegate_seat!(WlState);
delegate_pointer!(WlState);

impl ProvidesRegistryState for WlState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    smithay_client_toolkit::registry_handlers!(SeatState);
}
