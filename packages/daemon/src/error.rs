use thiserror::Error;

/// Unified error type for the entire waio-daemon.
///
/// Replaces the fragmented `RenderError`, `AuraError` (partial),
/// and unused `DaemonError` with a single type.
#[derive(Debug, Error)]
pub enum WaioError {
    #[error("Wayland error: {0}")]
    Wayland(#[from] anyhow::Error),

    #[error("Slint compilation failed: {0}")]
    SlintCompilation(String),

    #[error("Slint render failed: {0}")]
    SlintRender(String),

    #[error("Lua error: {0}")]
    Lua(#[from] mlua::Error),

    #[error("Aura not found: {0}")]
    AuraNotFound(String),

    #[error("Surface state error: {0}")]
    #[allow(dead_code)]
    SurfaceState(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Aura error: {0}")]
    Aura(#[from] waio_shared::entity::AuraError),
}

/// Type alias for daemon results.
pub type Result<T> = std::result::Result<T, WaioError>;
