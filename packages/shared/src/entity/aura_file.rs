use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::entity::{Aura, AuraConfig, AuraLayer, AuraType, LayerAnchor, Size};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraFile {
    pub meta: AuraMeta,
    pub config: AuraConfigYaml,
    pub slint_code: String,
    pub lua_code: String,
    /// Optional layout block for sub-component rendering.
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraMeta {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuraConfigYaml {
    pub anchor: String,
    pub width: u32,
    pub height: u32,
    pub exclusive_zone: i32,
    #[serde(default)]
    pub margin: Margin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Margin {
    #[serde(default)]
    pub top: i32,
    #[serde(default)]
    pub left: i32,
    #[serde(default)]
    pub right: i32,
    #[serde(default)]
    pub bottom: i32,
}

impl AuraFile {
    pub fn from_path(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_content(&content)
    }

    pub fn from_content(content: &str) -> anyhow::Result<Self> {
        // Extract YAML from <yaml>...</yaml> block
        let yaml_part = extract_block(content, "yaml")
            .ok_or_else(|| anyhow::anyhow!("<yaml> block not found"))?;

        let yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_part)?;

        let meta = yaml
            .get("meta")
            .and_then(|v| serde_yaml::from_value(v.clone()).ok())
            .ok_or_else(|| anyhow::anyhow!("Missing meta block"))?;

        let config = yaml
            .get("config")
            .and_then(|v| serde_yaml::from_value(v.clone()).ok())
            .unwrap_or_default();

        let slint_code = extract_block(content, "slint")
            .ok_or_else(|| anyhow::anyhow!("<slint> block not found"))?;

        let lua_code = extract_block(content, "lua")
            .ok_or_else(|| anyhow::anyhow!("<lua> block not found"))?;

        // Optional <layout> block for sub-component rendering.
        let layout = extract_block(content, "layout");

        Ok(Self {
            meta,
            config,
            slint_code,
            lua_code,
            layout,
        })
    }

    pub fn to_aura(&self) -> Aura {
        // Parse layers from layout block if present.
        let layers = self
            .layout
            .as_ref()
            .and_then(|l| {
                parse_layers_yaml(l).ok()
            })
            .unwrap_or_default();

        Aura {
            id: self.meta.id.clone(),
            name: self.meta.name.clone(),
            aura_type: AuraType::Bar,
            slint_code: self.slint_code.clone(),
            lua_code: Some(self.lua_code.clone()),
            config: AuraConfig {
                anchor: LayerAnchor::from_str(&self.config.anchor),
                size: Size {
                    width: self.config.width,
                    height: self.config.height,
                },
                exclusive_zone: self.config.exclusive_zone,
            },
            layers,
        }
    }
}

/// Parse layer definitions from the <layout> YAML block.
///
/// Format:
/// ```yaml
/// Background: { x: 0, y: 0 }
/// TimeLayer: { x: 10, y: 10 }
/// DateLayer: { x: 150, y: 12 }
/// ```
fn parse_layers_yaml(layout: &str) -> anyhow::Result<Vec<AuraLayer>> {
    let layers_map: serde_yaml::Value = serde_yaml::from_str(layout)?;

    let mut layers = Vec::new();
    if let serde_yaml::Value::Mapping(map) = layers_map {
        for (key, value) in map {
            let name = key.as_str().unwrap_or("").to_string();
            let mut x = 0u32;
            let mut y = 0u32;
            let mut layer_w = 1920u32;
            let mut layer_h = 40u32;
            let mut dynamic = true;

            if let serde_yaml::Value::Mapping(props) = value {
                if let Some(v) = props.get("x") {
                    x = v.as_u64().unwrap_or(0) as u32;
                }
                if let Some(v) = props.get("y") {
                    y = v.as_u64().unwrap_or(0) as u32;
                }
                if let Some(v) = props.get("w") {
                    layer_w = v.as_u64().unwrap_or(1920) as u32;
                }
                if let Some(v) = props.get("h") {
                    layer_h = v.as_u64().unwrap_or(40) as u32;
                }
                if let Some(v) = props.get("dynamic") {
                    dynamic = v.as_bool().unwrap_or(true);
                }
            }

            layers.push(AuraLayer { name, x, y, w: layer_w, h: layer_h, dynamic });
        }
    }

    // Sort: non-dynamic (static) layers first so they render as background.
    layers.sort_by_key(|l| l.dynamic);

    Ok(layers)
}

fn extract_block(content: &str, tag: &str) -> Option<String> {
    let pattern = format!(r"<{}>(?s)(.*?)</{}>", tag, tag);
    let re = Regex::new(&pattern).ok()?;

    re.captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}
