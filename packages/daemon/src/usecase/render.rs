use waio_shared::entity::Aura;

pub trait Renderer {
    fn init(&self) -> Result<(), RenderError>;
    fn render_aura(&self, aura: &Aura) -> Result<(), RenderError>;
    fn remove_aura(&self, aura_id: &str) -> Result<(), RenderError>;
    fn update_aura(&self, aura_id: &str, slint_code: &str) -> Result<(), RenderError>;
    fn shutdown(&self) -> Result<(), RenderError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Wayland error: {0}")]
    WaylandError(String),
    #[error("Slint compilation failed: {0}")]
    SlintCompilationFailed(String),
    #[error("Lua error: {0}")]
    LuaError(String),
    #[error("Rendering failed: {0}")]
    RenderFailed(String),
    #[error("Aura not found: {0}")]
    AuraNotFound(String),
}

pub struct RenderUseCase<R: Renderer> {
    pub renderer: R,
}

impl<R: Renderer> RenderUseCase<R> {
    pub fn new(renderer: R) -> Self {
        Self { renderer }
    }

    pub fn init(&self) -> Result<(), RenderError> {
        self.renderer.init()
    }

    pub fn render_aura(&self, aura: &Aura) -> Result<(), RenderError> {
        self.renderer.render_aura(aura)
    }

    pub fn shutdown(&self) -> Result<(), RenderError> {
        self.renderer.shutdown()
    }
}
