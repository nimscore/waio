use anyhow::Result;
use smithay_client_toolkit::compositor::CompositorState;
use smithay_client_toolkit::reexports::client::protocol::wl_shm;
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::QueueHandle;
use smithay_client_toolkit::shell::wlr_layer::{Anchor, Layer, LayerShell, LayerSurface};
use smithay_client_toolkit::shm::slot::SlotPool;
use smithay_client_toolkit::shm::Shm;

use crate::infrastructure::wayland::WlState;
use waio_shared::entity::{AuraConfig, LayerAnchor};

pub struct AuraSurface {
    pub surface: WlSurface,
    pub layer_surface: LayerSurface,
    pub pool: SlotPool,
    pub width: u32,
    pub height: u32,
}

impl AuraSurface {
    pub fn new(
        compositor: &CompositorState,
        layer_shell: &LayerShell,
        shm: &Shm,
        qh: &QueueHandle<WlState>,
        config: &AuraConfig,
    ) -> Result<Self> {
        let surface = compositor.create_surface(qh);
        let layer_surface = layer_shell.create_layer_surface(
            qh,
            surface.clone(),
            Layer::Top,
            Some("waio-aura"),
            None,
        );

        let anchor = match config.anchor {
            LayerAnchor::Top => Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            LayerAnchor::Bottom => Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            LayerAnchor::Left => Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT,
            LayerAnchor::Right => Anchor::TOP | Anchor::BOTTOM | Anchor::RIGHT,
        };
        layer_surface.set_anchor(anchor);
        layer_surface.set_size(config.size.width, config.size.height);
        layer_surface.set_exclusive_zone(config.exclusive_zone as i32);

        // Initial commit without buffer - compositor will respond with configure
        surface.commit();

        let pool = SlotPool::new((config.size.width * config.size.height * 4) as usize, shm)?;

        Ok(Self {
            surface,
            layer_surface,
            pool,
            width: config.size.width,
            height: config.size.height,
        })
    }

    pub fn draw<F>(&mut self, draw_fn: F) -> Result<()>
    where
        F: FnOnce(&mut [u8], u32, u32),
    {
        let width = self.width;
        let height = self.height;
        let stride = width * 4;

        let slot = self.pool.new_slot((height * stride) as usize)?;
        let buffer = self.pool.create_buffer_in(
            &slot,
            width as i32,
            height as i32,
            stride as i32,
            wl_shm::Format::Argb8888,
        )?;

        if let Some(canvas) = buffer.canvas(&mut self.pool) {
            draw_fn(canvas, width, height);
        }

        buffer.attach_to(&self.surface)?;
        self.surface
            .damage_buffer(0, 0, width as i32, height as i32);
        self.surface.commit();

        Ok(())
    }

    pub fn commit(&self) {
        self.surface.commit();
    }
}
