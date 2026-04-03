#![allow(dead_code)]

use std::path::PathBuf;
use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use anyhow::Result;
use waio_shared::protocol::{JsonRpcRequest, JsonRpcResponse, JsonRpcError};

pub struct IpcServer {
    socket_path: PathBuf,
}

impl IpcServer {
    pub fn new() -> Self {
        let socket_path = std::env::var("XDG_RUNTIME_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/tmp"))
            .join("waio.sock");

        if socket_path.exists() {
            if !is_socket_in_use(&socket_path) {
                let _ = std::fs::remove_file(&socket_path);
            }
        }

        Self { socket_path }
    }

    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }

    pub async fn run<F, Fut>(&self, handler: F) -> Result<()>
    where
        F: Fn(JsonRpcRequest) -> Fut + Send + Sync + Clone + 'static,
        Fut: std::future::Future<Output = JsonRpcResponse> + Send,
    {
        let listener = UnixListener::bind(&self.socket_path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                &self.socket_path,
                std::fs::Permissions::from_mode(0o600)
            )?;
        }

        tracing::info!("IPC server listening on {}", self.socket_path.display());

        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let handler = handler.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, handler).await {
                            tracing::error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }
    }
}

async fn handle_connection<F, Fut>(stream: UnixStream, handler: F) -> anyhow::Result<()>
where
    F: Fn(JsonRpcRequest) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = JsonRpcResponse>,
{
    let (read_half, mut write_half) = stream.into_split();
    let mut reader = BufReader::new(read_half);

    let mut line = String::new();
    while reader.read_line(&mut line).await? > 0 {
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let error_response = JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: "Parse error".to_string(),
                        data: Some(e.to_string()),
                    }),
                    id: 0,
                };
                let response_json = serde_json::to_string(&error_response)?;
                write_half.write_all(response_json.as_bytes()).await?;
                write_half.write_all(b"\n").await?;
                line.clear();
                continue;
            }
        };

        let response = handler(request).await;
        let response_json = serde_json::to_string(&response)?;

        write_half.write_all(response_json.as_bytes()).await?;
        write_half.write_all(b"\n").await?;

        line.clear();
    }

    Ok(())
}

#[cfg(unix)]
fn is_socket_in_use(path: &PathBuf) -> bool {
    use std::os::unix::net::UnixStream;
    UnixStream::connect(path).is_ok()
}