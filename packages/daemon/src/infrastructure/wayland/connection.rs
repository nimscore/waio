use std::rc::Rc;

use anyhow::Result;
use smithay_client_toolkit::compositor::{CompositorHandler, CompositorState};
use smithay_client_toolkit::output::{OutputHandler, OutputState};
use smithay_client_toolkit::reexports::client::globals::registry_queue_init;
use smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::{Connection, EventQueue, QueueHandle};
use smithay_client_toolkit::registry::{ProvidesRegistryState, RegistryState};
use smithay_client_toolkit::shell::wlr_layer::{
    LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceConfigure,
};
use smithay_client_toolkit::shm::{Shm, ShmHandler};
use smithay_client_toolkit::{
    delegate_compositor, delegate_layer, delegate_output, delegate_registry, delegate_shm,
};
use tracing::{debug, info};

use crate::infrastructure::slint::SlintRenderer;

pub struct WlState {
    pub registry_state: RegistryState,
    pub output_state: OutputState,
    pub compositor_state: CompositorState,
    pub layer_shell: LayerShell,
    pub shm: Shm,
    /// Optional reference to the renderer. Set after renderer is created.
    pub renderer: Option<Rc<SlintRenderer>>,
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

        let wl_state = WlState {
            registry_state: RegistryState::new(&globals),
            output_state,
            compositor_state: compositor_state.clone(),
            layer_shell,
            shm,
            renderer: None,
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

    fn new_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {}

    fn update_output(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {}

    fn output_destroyed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, _output: WlOutput) {
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
            configure.new_size.0,
            configure.new_size.1,
            serial
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

delegate_compositor!(WlState);
delegate_output!(WlState);
delegate_layer!(WlState);
delegate_registry!(WlState);
delegate_shm!(WlState);

impl ProvidesRegistryState for WlState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    smithay_client_toolkit::registry_handlers!();
}
