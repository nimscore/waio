use std::path::PathBuf;
use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use waio_shared::protocol::{JsonRpcRequest, JsonRpcResponse, DaemonMethod};

pub struct IpcClient {
    socket_path: PathBuf,
}

impl IpcClient {
    pub fn new() -> Self {
        let socket_path = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"))
            .join("waio.sock");

        Self { socket_path }
    }

    pub async fn send(&self, method: DaemonMethod) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: match &method {
                DaemonMethod::LoadAura { .. } => "aura.load",
                DaemonMethod::UnloadAura { .. } => "aura.unload",
                DaemonMethod::UpdateAura { .. } => "aura.update",
                DaemonMethod::ListAuras => "aura.list",
                DaemonMethod::SystemInfo => "system.info",
                DaemonMethod::SystemShutdown => "system.shutdown",
            }.to_string(),
            params: serde_json::to_value(&method)?,
            id: 1,
        };

        let request_json = serde_json::to_string(&request)?;
        stream.write_all(request_json.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();
        
        use tokio::time::{timeout, Duration};
        let result = timeout(Duration::from_secs(5), reader.read_line(&mut response_line)).await;
        
        match result {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return Err(Box::new(e)),
            Err(_) => return Err("Timeout waiting for response".into()),
        }

        let response: JsonRpcResponse = serde_json::from_str(&response_line)?;
        Ok(response)
    }
}