use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Default config YAML used when no user config exists.
const DEFAULT_CONFIG_YAML: &str = r#"# Waio Configuration
# Location: ~/.config/waio/config.yaml
# Override: WAIO_CONFIG=/path/to/config.yaml

# IPC socket path (default: $XDG_RUNTIME_DIR/waio.sock based on UID)
# socket_path: "/run/user/1000/waio.sock"

# Log level (default: "info,waio_daemon=debug")
log_level: "info,waio_daemon=debug"

# Persistent data directory (default: $XDG_DATA_HOME/waio)
# data_dir: "/home/user/.local/share/waio"

# Default output (monitor) name, or null for auto-detect
# default_output: null
"#;

/// Unified configuration for both daemon and CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaioConfig {
    /// IPC socket path for daemon communication.
    #[serde(default)]
    pub socket_path: Option<String>,

    /// Log level filter (daemon only).
    #[serde(default)]
    pub log_level: Option<String>,

    /// Persistent data directory (daemon only).
    #[serde(default)]
    pub data_dir: Option<String>,

    /// Default output (monitor) name for aura binding.
    #[serde(default)]
    pub default_output: Option<String>,
}

impl WaioConfig {
    /// Default socket path based on XDG_RUNTIME_DIR or fallback.
    pub fn default_socket_path() -> String {
        let uid = unsafe { libc::getuid() };
        format!("/run/user/{}/waio.sock", uid)
    }

    /// Default data directory.
    pub fn default_data_dir() -> String {
        dirs::data_local_dir()
            .map(|d| d.join("waio").to_string_lossy().to_string())
            .unwrap_or_else(|| "/tmp/waio".to_string())
    }

    /// Default log level.
    pub fn default_log_level() -> String {
        "info,waio_daemon=debug".to_string()
    }

    /// Load config from environment or default path.
    /// If the file does not exist, creates the directory and writes the default config.
    pub fn load() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        // Try explicit env variable first.
        if let Ok(path_str) = std::env::var("WAIO_CONFIG") {
            let path = PathBuf::from(path_str);
            return Self::from_path(&path);
        }

        // XDG config path: ~/.config/waio/config.yaml
        let config_path = dirs::config_dir()
            .map(|d| d.join("waio").join("config.yaml"))
            .unwrap_or_else(|| PathBuf::from("/tmp/waio/config.yaml"));

        if config_path.exists() {
            return Self::from_path(&config_path);
        }

        // File does not exist — create directory and write default config.
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&config_path, DEFAULT_CONFIG_YAML)?;
        eprintln!(
            "Config file created at: {}",
            config_path.display()
        );

        Self::from_path(&config_path)
    }

    /// Load config from a specific YAML file.
    pub fn from_path(path: &std::path::Path) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    /// Resolved socket path (config value or system default).
    pub fn socket_path(&self) -> String {
        self.socket_path.clone()
            .unwrap_or_else(|| Self::default_socket_path())
    }

    /// Resolved log level (config value or default).
    pub fn log_level(&self) -> String {
        self.log_level.clone()
            .unwrap_or_else(|| Self::default_log_level())
    }

    /// Resolved data directory (config value or default).
    pub fn data_dir(&self) -> String {
        self.data_dir.clone()
            .unwrap_or_else(|| Self::default_data_dir())
    }
}

impl Default for WaioConfig {
    fn default() -> Self {
        Self {
            socket_path: None,
            log_level: None,
            data_dir: None,
            default_output: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_parses() {
        let config: WaioConfig = serde_yaml::from_str(DEFAULT_CONFIG_YAML)
            .expect("Default config YAML should parse");
        // Optional paths are null — resolved programmatically.
        assert!(config.socket_path.is_none());
        assert!(config.data_dir.is_none());
        assert!(config.log_level.is_some());
    }

    #[test]
    fn test_empty_config_parses() {
        let config: WaioConfig = serde_yaml::from_str("").expect("Empty config should parse with defaults");
        assert!(config.socket_path.is_none());
        assert!(config.log_level.is_none());
        assert!(config.data_dir.is_none());
        assert!(config.default_output.is_none());
    }
}
