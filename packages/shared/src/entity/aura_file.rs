use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::entity::{Aura, AuraConfig, AuraType, LayerAnchor, Size};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraFile {
    pub meta: AuraMeta,
    pub config: AuraConfigYaml,
    pub slint_code: String,
    pub lua_code: String,
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
        // Extract YAML part - from after first "---" to before "<slint>"
        let yaml_part = content
            .split("---")
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Invalid aura file: no --- delimiter"))?
            .split("<slint>")
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid aura file: no <slint> block"))?
            .trim();

        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_part)?;

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

        Ok(Self {
            meta,
            config,
            slint_code,
            lua_code,
        })
    }

    pub fn to_aura(&self) -> Aura {
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
        }
    }
}

fn extract_block(content: &str, tag: &str) -> Option<String> {
    let pattern = format!(r"<{}>(?s)(.*?)</{}>", tag, tag);
    let re = Regex::new(&pattern).ok()?;

    re.captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}
