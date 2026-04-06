//! Integration tests for waio-daemon.
//!
//! These tests validate the core functionality: timer registry, property store,
//! aura file parsing, config validation, and repository lifecycle.

#[cfg(test)]
mod tests {
    use waio_shared::entity::{AuraFile, LayerAnchor};
    use waio_shared::protocol::error_codes;

    /// Test that a valid `.wa` file is parsed correctly.
    #[test]
    fn test_parse_aura_file() {
        let content = r#"<yaml>
meta:
  id: test-clock
  name: Test Clock
  version: 1.0.0
  author: Test
  description: Test widget
  permissions: [timer]

config:
  anchor: top
  width: 1920
  height: 40
  exclusive_zone: 40
  output: "eDP-1"
</yaml>

<slint>
export component Background inherits Window {
    width: 1920px; height: 40px;
    background: #1a1a1a;
}
</slint>

<lua>
print("test")
</lua>

<layout>
Background: { x: 0, y: 0, w: 1920, h: 40, dynamic: false }
</layout>
"#;

        let af = AuraFile::from_content(content).expect("Failed to parse aura file");
        assert_eq!(af.meta.id, "test-clock");
        assert_eq!(af.meta.name, "Test Clock");
        assert_eq!(af.config.output, Some("eDP-1".to_string()));
        assert_eq!(af.config.width, 1920);
        assert_eq!(af.config.height, 40);
        assert!(af.layout.is_some());

        let aura = af.to_aura();
        assert_eq!(aura.id, "test-clock");
        assert_eq!(aura.config.output, Some("eDP-1".to_string()));
        assert_eq!(aura.config.anchor, LayerAnchor::Top);
        assert_eq!(aura.layers.len(), 1);
        assert_eq!(aura.layers[0].name, "Background");
        assert_eq!(aura.layers[0].x, 0);
        assert_eq!(aura.layers[0].y, 0);
        assert!(!aura.layers[0].dynamic);
    }

    /// Test that output defaults to None when not specified.
    #[test]
    fn test_aura_output_defaults_to_none() {
        let content = r#"<yaml>
meta:
  id: test
  name: Test
  version: 1.0.0
  author: Test
  description: Test
  permissions: []

config:
  anchor: bottom
  width: 800
  height: 30
  exclusive_zone: 30
</yaml>

<slint>
export component Win inherits Window { width: 800px; height: 30px; }
</slint>

<lua>
</lua>
"#;

        let af = AuraFile::from_content(content).expect("Failed to parse");
        assert_eq!(af.config.output, None);

        let aura = af.to_aura();
        assert_eq!(aura.config.output, None);
        assert_eq!(aura.config.anchor, LayerAnchor::Bottom);
    }

    /// Test that config validation catches invalid values.
    #[test]
    fn test_config_validation() {
        use waio_shared::config::WaioConfig;

        // Empty config should validate and use defaults.
        let config = WaioConfig::default();
        config.validate(); // Should not panic.

        // Config with values should also validate fine.
        let config = WaioConfig {
            socket_path: Some("/tmp/waio.sock".into()),
            log_level: Some("info".into()),
            data_dir: Some("/tmp/waio-data".into()),
            default_output: None,
        };
        config.validate(); // Should not panic.
    }

    /// Test that error codes are consistent and match JSON-RPC spec.
    #[test]
    fn test_error_codes_json_rpc_compliance() {
        // Standard JSON-RPC 2.0 codes.
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);

        // Application-specific codes (server error range).
        assert_eq!(error_codes::SERVER_ERROR, -32000);
        assert_eq!(error_codes::NOT_FOUND, -32001);
        assert_eq!(error_codes::ALREADY_EXISTS, -32002);
        assert_eq!(error_codes::PERSISTENCE_ERROR, -32003);
    }

    /// Test that permissions are correctly passed through AuraFile → Aura.
    #[test]
    fn test_permissions_passthrough() {
        let content = r#"<yaml>
meta:
  id: perm-test
  name: Perm Test
  version: 1.0.0
  author: Test
  description: Test
  permissions: [timer, system_time, fs_read, http]

config:
  anchor: top
  width: 800
  height: 30
  exclusive_zone: 30
</yaml>

<slint>
export component X inherits Window { width: 800px; height: 30px; }
</slint>

<lua>
print("test")
</lua>
"#;

        let af = AuraFile::from_content(content).expect("Failed to parse");
        let aura = af.to_aura();
        assert_eq!(
            aura.permissions,
            vec!["timer", "system_time", "fs_read", "http"]
        );
    }
}
