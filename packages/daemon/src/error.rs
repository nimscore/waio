#![allow(dead_code)]

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("Wayland error: {0}")]
    Wayland(String),
    #[error("Slint error: {0}")]
    Slint(String),
    #[error("Lua error: {0}")]
    Lua(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
