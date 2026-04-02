use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuraError {
    #[error("Invalid aura syntax: {0}")]
    InvalidSyntax(String),
    #[error("Aura not found: {0}")]
    NotFound(String),
    #[error("Compilation failed: {0}")]
    CompilationFailed(String),
    #[error("Lua error: {0}")]
    LuaError(String),
}
