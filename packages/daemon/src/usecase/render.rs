#![allow(dead_code)]

use crate::error::{Result, WaioError};
use waio_shared::entity::Aura;

pub trait Renderer {
    fn init(&self) -> Result<()>;
    fn render_aura(&self, aura: &Aura) -> Result<()>;
    fn remove_aura(&self, aura_id: &str) -> Result<()>;
    fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<()>;
    fn shutdown(&self) -> Result<()>;
}

impl From<String> for WaioError {
    fn from(e: String) -> Self {
        WaioError::SlintRender(e)
    }
}

impl From<&str> for WaioError {
    fn from(e: &str) -> Self {
        WaioError::SlintRender(e.to_string())
    }
}

pub struct RenderUseCase<R: Renderer> {
    pub renderer: R,
}

impl<R: Renderer> RenderUseCase<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    pub fn init(&self) -> Result<()> {
        self.renderer.init()
    }

    pub fn render_aura(&self, aura: &Aura) -> Result<()> {
        self.renderer.render_aura(aura)
    }

    pub fn shutdown(&self) -> Result<()> {
        self.renderer.shutdown()
    }
}
