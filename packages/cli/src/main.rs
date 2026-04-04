use clap::{Parser, Subcommand};
use std::path::PathBuf;
use waio_shared::protocol::DaemonMethod;

mod ipc_client;

use ipc_client::IpcClient;

#[derive(Parser)]
#[command(name = "waio")]
#[command(about = "Waio Aura Manager CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Load an Aura from a .wa file
    Load {
        /// Path to the .wa file
        path: PathBuf,
    },
    /// Update a loaded Aura from a .wa file
    Update {
        /// ID of the loaded Aura
        id: String,
        /// Path to the new .wa file
        path: PathBuf,
    },
    /// Unload an Aura by ID
    Unload {
        /// ID of the Aura to unload
        id: String,
    },
    /// List loaded Auras
    List,
    /// Show daemon info
    Info,
    /// Stop the daemon
    Stop,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = IpcClient::new();

    let result = match cli.command {
        Commands::Load { path } => {
            let method = DaemonMethod::LoadAura {
                source: "file".to_string(),
                path: Some(path.to_string_lossy().to_string()),
                content: None,
                id: path.file_stem().unwrap().to_string_lossy().to_string(),
            };
            client.send(method).await
        }
        Commands::Update { id, path } => {
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| {
                    eprintln!("\x1b[31mError\x1b[0m: Failed to read {}: {}", path.display(), e);
                    std::process::exit(1);
                });
            let method = DaemonMethod::UpdateAura { id, content };
            client.send(method).await
        }
        Commands::Unload { id } => {
            let method = DaemonMethod::UnloadAura { id };
            client.send(method).await
        }
        Commands::List => {
            let method = DaemonMethod::ListAuras;
            client.send(method).await
        }
        Commands::Info => {
            let method = DaemonMethod::SystemInfo;
            client.send(method).await
        }
        Commands::Stop => {
            let method = DaemonMethod::SystemShutdown;
            client.send(method).await
        }
    };

    match result {
        Ok(resp) => {
            if let Some(error) = resp.error {
                eprintln!("\x1b[31mError\x1b[0m: {} ({})", error.message, error.code);
                if let Some(data) = error.data {
                    eprintln!("{}", data);
                }
                std::process::exit(1);
            }
            
            if let Some(result) = resp.result {
                println!("\x1b[32mSuccess\x1b[0m: {}", serde_json::to_string_pretty(&result).unwrap());
            }
        }
        Err(e) => {
            eprintln!("\x1b[31mFailed to connect to daemon\x1b[0m: {}", e);
            eprintln!("Make sure the daemon is running (waio-daemon)");
            std::process::exit(1);
        }
    }
}