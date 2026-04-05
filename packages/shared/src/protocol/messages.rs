use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum DaemonMethod {
    #[serde(rename = "aura.load")]
    LoadAura {
        /// Optional aura ID. If None, the daemon uses meta.id from the .wa file.
        #[serde(default)]
        id: Option<String>,
        source: String,
        path: Option<String>,
        content: Option<String>,
    },

    #[serde(rename = "aura.unload")]
    UnloadAura { id: String },

    #[serde(rename = "aura.update")]
    UpdateAura { id: String, content: String },

    #[serde(rename = "system.status")]
    SystemStatus,

    #[serde(rename = "system.shutdown")]
    SystemShutdown,
}

pub fn rpc_success(result: serde_json::Value, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(result),
        error: None,
        id,
    }
}

pub fn rpc_error(code: i32, message: &str, data: Option<String>, id: u64) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: None,
        error: Some(JsonRpcError {
            code,
            message: message.to_string(),
            data: data.map(String::from),
        }),
        id,
    }
}
