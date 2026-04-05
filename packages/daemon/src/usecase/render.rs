use crate::error::{Result, WaioError};
use waio_shared::entity::Aura;

pub trait Renderer {
    #[allow(dead_code)]
    fn init(&self) -> Result<()>;
    #[allow(dead_code)]
    fn render_aura(&self, aura: &Aura) -> Result<()>;
    #[allow(dead_code)]
    fn remove_aura(&self, aura_id: &str) -> Result<()>;
    #[allow(dead_code)]
    fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<()>;
    #[allow(dead_code)]
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
