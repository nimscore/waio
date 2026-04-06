use serde::{Deserialize, Serialize};

/// Current IPC protocol version. Increment on breaking changes.
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Standard JSON-RPC 2.0 error codes.
pub mod error_codes {
    /// Invalid JSON was received by the server.
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid Request object.
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method does not exist.
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid method parameter(s).
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal JSON-RPC error.
    pub const INTERNAL_ERROR: i32 = -32603;
    /// Server error — application-specific codes start here.
    pub const SERVER_ERROR: i32 = -32000;
    /// Aura not found.
    pub const NOT_FOUND: i32 = -32001;
    /// Aura already loaded.
    pub const ALREADY_EXISTS: i32 = -32002;
    /// Persistence/storage error.
    pub const PERSISTENCE_ERROR: i32 = -32003;
}

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
