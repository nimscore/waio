use anyhow::Result;
use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::reexports::client::protocol::wl_shm;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::shell::wlr_layer::{
    Anchor, KeyboardInteractivity, Layer, LayerShell, LayerSurface, LayerSurfaceConfigure,
};
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::shm::Shm;

use crate::infrastructure::wayland::WlState;
use waio_shared::entity::{AuraConfig, LayerAnchor};

/// State machine for a layer surface lifecycle.
///
/// The wlr-layer-shell protocol requires a strict sequence:
/// 1. create surface → set_anchor → set_size → commit (NO buffer)
/// 2. compositor sends configure event with confirmed sizes
/// 3. ack_configure (automatic in SCTK) → attach buffer → damage → frame → commit
/// 4. frame callback → repeat step 3 for each new frame
///
/// Violating this sequence causes protocol error 2 (invalid_anchor / already_constructed).
#[derive(Debug)]
pub enum SurfaceState {
    /// Surface created, initial commit sent, waiting for compositor configure.
    Pending {
        surface: WlSurface,
        layer_surface: LayerSurface,
    },
    /// Configure received from compositor, sizes confirmed, ready to draw first frame.
    Configured {
        surface: WlSurface,
        layer_surface: LayerSurface,
        /// Dimensions confirmed by compositor (may differ from requested).
        configured_size: (u32, u32),
    },
    /// Active rendering, awaiting frame callback or dirty flag.
    Rendering {
        surface: WlSurface,
        layer_surface: LayerSurface,
        current_size: (u32, u32),
        /// Whether the surface needs redrawing.
        dirty: bool,
    },
    /// Surface closed by compositor or removed by user.
    Closed,
}

/// A Wayland layer surface with SHM-backed rendering,
/// implementing the correct wlr-layer-shell lifecycle.
pub struct AuraSurface {
    pub id: String,
    state: SurfaceState,
    /// Shared memory pool for buffer allocation.
    pool: SlotPool,
    /// Queue handle for frame callbacks.
    qh: QueueHandle<WlState>,
}

impl AuraSurface {
    /// Phase 1: Create the layer surface and send the initial commit (without a buffer).
    ///
    /// This triggers the compositor to send back a `configure` event with confirmed sizes.
    /// The surface enters `Pending` state and must NOT be drawn until `on_configure` is called.
    pub fn new(
        compositor: &CompositorState,
        layer_shell: &LayerShell,
        shm: &Shm,
        qh: &QueueHandle<WlState>,
        id: String,
        config: &AuraConfig,
    ) -> Result<Self> {
        tracing::info!(
            "Creating AuraSurface: id={}, requested={}x{}, anchor={:?}, exclusive_zone={}",
            id,
            config.size.width,
            config.size.height,
            config.anchor,
            config.exclusive_zone
        );

        let surface = compositor.create_surface(qh);

        let layer_surface = layer_shell.create_layer_surface(
            qh,
            surface.clone(),
            Layer::Top,
            Some("waio-aura"),
            None, // output — compositor will auto-select
        );

        let anchor = Self::anchor_to_bits(&config.anchor);
        layer_surface.set_anchor(anchor);
        layer_surface.set_size(config.size.width, config.size.height);
        layer_surface.set_exclusive_zone(config.exclusive_zone as i32);
        layer_surface.set_keyboard_interactivity(KeyboardInteractivity::None);

        // Initial commit WITHOUT buffer — signals readiness to receive configure.
        // This is critical: attaching a buffer before configure causes protocol error 2.
        surface.commit();

        let pool_size = config.size.width.max(1) * config.size.height.max(1) * 4;
        let pool = SlotPool::new(pool_size as usize, shm)?;

        Ok(Self {
            state: SurfaceState::Pending {
                surface,
                layer_surface,
            },
            pool,
            qh: qh.clone(),
            id,
        })
    }

    /// Called from `LayerShellHandler::configure` when the compositor sends size updates.
    ///
    /// SCTK automatically calls `ack_configure(serial)` BEFORE invoking our handler,
    /// so we do NOT need to call it manually. See smithay-client-toolkit source.
    ///
    /// Returns `true` if this is the FIRST configure (signals to render the first frame).
    pub fn on_configure(&mut self, configure: LayerSurfaceConfigure) -> bool {
        let new_width = configure.new_size.0.max(1);
        let new_height = configure.new_size.1.max(1);
        let new_size = (new_width, new_height);

        tracing::info!(
            "AuraSurface on_configure: id={}, size={}x{}",
            self.id,
            new_width,
            new_height
        );

        match std::mem::replace(&mut self.state, SurfaceState::Closed) {
            SurfaceState::Pending {
                surface,
                layer_surface,
            } => {
                // First configure — transition to Configured state.
                self.state = SurfaceState::Configured {
                    surface,
                    layer_surface,
                    configured_size: new_size,
                };
                true
            }
            SurfaceState::Configured {
                surface,
                layer_surface,
                configured_size: _,
            } => {
                // Subsequent configure while already configured — update sizes.
                self.state = SurfaceState::Configured {
                    surface,
                    layer_surface,
                    configured_size: new_size,
                };
                false
            }
            SurfaceState::Rendering {
                surface,
                layer_surface,
                current_size: _,
                dirty: _,
            } => {
                // Resize during rendering — mark dirty for next frame.
                self.state = SurfaceState::Rendering {
                    surface,
                    layer_surface,
                    current_size: new_size,
                    dirty: true,
                };
                false
            }
            SurfaceState::Closed => {
                self.state = SurfaceState::Closed;
                false
            }
        }
    }

    /// Phase 2: Render the first frame after receiving configure.
    ///
    /// The `render_fn` closure receives a mutable ARGB8888 canvas and the configured dimensions.
    /// After drawing, this method requests a frame callback and transitions to `Rendering` state.
    pub fn render_first_frame<F>(&mut self, render_fn: F) -> Result<()>
    where
        F: FnOnce(&mut [u8], u32, u32),
    {
        let (surface, layer_surface, size) = match &self.state {
            SurfaceState::Configured {
                surface,
                layer_surface,
                configured_size,
            } => (surface.clone(), layer_surface.clone(), *configured_size),
            _ => {
                return Err(anyhow::anyhow!(
                    "render_first_frame called on non-Configured surface {}",
                    self.id
                ));
            }
        };

        self.draw(&surface, render_fn, size)?;

        // Request frame callback so compositor notifies us when it's ready for the next frame.
        surface.frame(&self.qh, surface.clone());

        self.state = SurfaceState::Rendering {
            surface,
            layer_surface,
            current_size: size,
            dirty: false,
        };

        tracing::debug!("AuraSurface {} transitioned to Rendering state", self.id);
        Ok(())
    }

    /// Render a subsequent frame.
    ///
    /// Only draws if the surface is marked as dirty. After drawing, requests a frame callback
    /// and clears the dirty flag.
    pub fn render_frame<F>(&mut self, render_fn: F) -> Result<()>
    where
        F: FnOnce(&mut [u8], u32, u32),
    {
        // Extract and clone everything upfront to avoid borrow conflicts.
        let (surface, size, dirty) = match &self.state {
            SurfaceState::Rendering {
                surface,
                current_size,
                dirty,
                ..
            } => (surface.clone(), *current_size, *dirty),
            SurfaceState::Configured { .. } => {
                // Configured but never rendered — treat as first frame.
                return self.render_first_frame(render_fn);
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "render_frame called on non-Rendering surface {}",
                    self.id
                ));
            }
        };

        if !dirty {
            return Ok(());
        }

        self.draw(&surface, render_fn, size)?;
        surface.frame(&self.qh, surface.clone());

        // Clear dirty flag.
        if let SurfaceState::Rendering { dirty, .. } = &mut self.state {
            *dirty = false;
        }

        Ok(())
    }

    /// Mark the surface as needing redraw on the next render_frame call.
    pub fn mark_dirty(&mut self) {
        if let SurfaceState::Rendering { dirty, .. } = &mut self.state {
            *dirty = true;
        }
    }

    /// Mark the surface as closed. Actual resource cleanup happens when the struct is dropped.
    pub fn close(&mut self) {
        self.state = SurfaceState::Closed;
        tracing::info!("AuraSurface {} marked as closed", self.id);
    }

    /// Check if the surface is in Rendering state.
    pub fn is_rendering(&self) -> bool {
        matches!(self.state, SurfaceState::Rendering { .. })
    }

    /// Get a reference to the underlying WlSurface, if it exists.
    pub fn surface(&self) -> Option<&WlSurface> {
        match &self.state {
            SurfaceState::Pending { surface, .. }
            | SurfaceState::Configured { surface, .. }
            | SurfaceState::Rendering { surface, .. } => Some(surface),
            SurfaceState::Closed => None,
        }
    }

    /// Get a reference to the underlying LayerSurface, if it exists.
    pub fn layer_surface(&self) -> Option<&LayerSurface> {
        match &self.state {
            SurfaceState::Pending { layer_surface, .. }
            | SurfaceState::Configured { layer_surface, .. }
            | SurfaceState::Rendering { layer_surface, .. } => Some(layer_surface),
            SurfaceState::Closed => None,
        }
    }

    // ─── Private helpers ───

    /// Send a pre-composited ARGB8888 canvas to the Wayland surface.
    /// Used by sub-component compositing mode.
    pub fn draw_precomposited(&mut self, canvas: &[u8]) -> Result<()> {
        let (w, h) = match &self.state {
            SurfaceState::Rendering { current_size, .. } => *current_size,
            SurfaceState::Configured { configured_size, .. } => *configured_size,
            _ => return Err(anyhow::anyhow!("Surface not in renderable state")),
        };

        let stride = w as i32 * 4;
        let (buffer, slot_canvas) = self.pool.create_buffer(w as i32, h as i32, stride, wl_shm::Format::Argb8888)?;

        // Copy the pre-composited canvas into the buffer.
        let copy_len = canvas.len().min(slot_canvas.len());
        slot_canvas[..copy_len].copy_from_slice(&canvas[..copy_len]);

        if let Some(surface) = self.surface() {
            surface.damage_buffer(0, 0, w as i32, h as i32);
            buffer.attach_to(surface)?;
            surface.commit();

            // Request next frame callback.
            surface.frame(&self.qh, surface.clone());
        }

        // Transition to Rendering state if we were in Configured.
        if matches!(self.state, SurfaceState::Configured { .. }) {
            if let SurfaceState::Configured { surface, layer_surface, configured_size } =
                std::mem::replace(&mut self.state, SurfaceState::Closed)
            {
                self.state = SurfaceState::Rendering {
                    surface,
                    layer_surface,
                    current_size: configured_size,
                    dirty: false,
                };
            }
        }

        tracing::debug!("AuraSurface {} precomposited frame sent: {}x{}", self.id, w, h);

        Ok(())
    }

    /// Low-level draw: create buffer → run render_fn → damage → attach → commit.
    fn draw<F>(&mut self, surface: &WlSurface, render_fn: F, (w, h): (u32, u32)) -> Result<()>
    where
        F: FnOnce(&mut [u8], u32, u32),
    {
        let stride = w as i32 * 4;
        let (buffer, canvas) = self.pool.create_buffer(w as i32, h as i32, stride, wl_shm::Format::Argb8888)?;

        tracing::debug!(
            "AuraSurface {} drawing: {}x{}, stride={}, canvas={} bytes",
            self.id,
            w,
            h,
            stride,
            canvas.len()
        );

        render_fn(canvas, w, h);

        // Mark the entire surface as damaged so the compositor knows to redraw it.
        surface.damage_buffer(0, 0, w as i32, h as i32);

        // Attach the buffer and commit to present the frame.
        buffer.attach_to(surface)?;
        surface.commit();

        Ok(())
    }

    /// Convert our LayerAnchor enum to the bit flags expected by the protocol.
    fn anchor_to_bits(anchor: &LayerAnchor) -> Anchor {
        match anchor {
            LayerAnchor::Top => Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            LayerAnchor::Bottom => Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            LayerAnchor::Left => Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT,
            LayerAnchor::Right => Anchor::TOP | Anchor::BOTTOM | Anchor::RIGHT,
            LayerAnchor::TopLeft => Anchor::TOP | Anchor::LEFT,
            LayerAnchor::TopRight => Anchor::TOP | Anchor::RIGHT,
            LayerAnchor::BottomLeft => Anchor::BOTTOM | Anchor::LEFT,
            LayerAnchor::BottomRight => Anchor::BOTTOM | Anchor::RIGHT,
        }
    }
}
