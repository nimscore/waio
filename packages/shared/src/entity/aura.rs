use serde::{Deserialize, Serialize};

use crate::entity::error::AuraError;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuraType {
    Bar,
    Widget,
    Overlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aura {
    pub id: String,
    pub name: String,
    pub aura_type: AuraType,
    pub slint_code: String,
    pub lua_code: Option<String>,
    pub config: AuraConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraConfig {
    pub anchor: LayerAnchor,
    pub size: Size,
    pub exclusive_zone: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LayerAnchor {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl LayerAnchor {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top" => LayerAnchor::Top,
            "bottom" => LayerAnchor::Bottom,
            "left" => LayerAnchor::Left,
            "right" => LayerAnchor::Right,
            "top-left" | "topleft" => LayerAnchor::TopLeft,
            "top-right" | "topright" => LayerAnchor::TopRight,
            "bottom-left" | "bottomleft" => LayerAnchor::BottomLeft,
            "bottom-right" | "bottomright" => LayerAnchor::BottomRight,
            _ => LayerAnchor::Top,
        }
    }
}

impl Aura {
    pub fn new(id: String, name: String, aura_type: AuraType, slint_code: String) -> Self {
        Self {
            id,
            name,
            aura_type,
            slint_code,
            lua_code: None,
            config: AuraConfig::default(),
        }
    }

    pub fn validate(&self) -> Result<(), AuraError> {
        if self.name.is_empty() || self.slint_code.is_empty() {
            return Err(AuraError::InvalidSyntax(
                "Name and Slint code required".into(),
            ));
        }
        Ok(())
    }
}

impl Default for Aura {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Untitled".into(),
            aura_type: AuraType::Bar,
            slint_code: DEFAULT_BAR_SLINT.to_string(),
            lua_code: None,
            config: AuraConfig::default(),
        }
    }
}

impl Default for AuraConfig {
    fn default() -> Self {
        Self {
            anchor: LayerAnchor::Top,
            size: Size {
                width: 1920,
                height: 30,
            },
            exclusive_zone: 30,
        }
    }
}

pub const DEFAULT_BAR_SLINT: &str = r#"
export component AuraBar inherits Window {
    width: 1920px;
    height: 40px;
    background: #2d2d2d;
    in property <string> time_text: "";
    in property <string> date_text: "";
    Text {
        text: parent.time_text;
        color: #ffffff;
        font-size: 18px;
        x: 10px;
        y: (parent.height - self.height) / 2;
    }
    Text {
        text: parent.date_text;
        color: #aaaaaa;
        font-size: 14px;
        x: 150px;
        y: (parent.height - self.height) / 2;
    }
}
"#;
