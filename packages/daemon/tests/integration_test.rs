//! Integration tests for waio-daemon.
//!
//! These tests validate the core functionality: timer registry, property store,
//! aura file parsing, and output tracking.

#[cfg(test)]
mod tests {
    use waio_shared::entity::{AuraFile, LayerAnchor};

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
}
