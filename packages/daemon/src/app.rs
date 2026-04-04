use crate::controller::IpcServer;
use crate::infrastructure::slint::SlintRenderer;
use crate::infrastructure::wayland::WaylandConnection;
use crate::usecase::render::Renderer;
use anyhow::Result;
use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use waio_shared::entity::{Aura, AuraFile};
use waio_shared::protocol::{
    DaemonMethod, JsonRpcRequest, JsonRpcResponse, rpc_error, rpc_success,
};

pub enum IpcCommand {
    Request(JsonRpcRequest, tokio::sync::oneshot::Sender<JsonRpcResponse>),
}

pub struct AppState {
    pub auras: Arc<Mutex<HashMap<String, Aura>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            auras: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

/// Main daemon entry point. Sets up Wayland, Slint renderer, calloop event loop,
/// IPC server, and runs the dispatch loop.
pub fn run() -> Result<()> {
    info!("Starting Waio Daemon...");
    init_logging();

    if std::env::var("WAYLAND_DISPLAY").is_err() {
        anyhow::bail!(
            "WAYLAND_DISPLAY not set. Try: WAYLAND_DISPLAY=wayland-0 cargo run -p waio-daemon"
        );
    }

    // 1. Connect to Wayland and get all the necessary state objects.
    let (wl_conn, event_queue, mut wl_state) = WaylandConnection::connect()?;

    // 2. Create the Slint renderer — it owns compositor and qh.
    //    LayerShell and Shm remain in WlState and are borrowed during load_aura.
    let renderer = Rc::new(SlintRenderer::new(
        wl_state.compositor_state.clone(),
        wl_conn.qh.clone(),
    ));

    // 3. Give the renderer to WlState so handlers can dispatch events to it.
    wl_state.renderer = Some(renderer.clone());

    // 4. Create the calloop event loop and insert WaylandSource into it.
    let mut event_loop: EventLoop<crate::infrastructure::wayland::WlState> =
        EventLoop::try_new().expect("Failed to create event loop");

    WaylandSource::new(wl_conn.conn.clone(), event_queue)
        .insert(event_loop.handle())
        .expect("Failed to insert WaylandSource");

    // 5. Set up IPC server on a separate thread.
    let state = AppState::new();
    let auras = state.auras.clone();
    let renderer_for_ipc = renderer.clone();

    let (tx, rx) = std::sync::mpsc::channel::<IpcCommand>();

    {
        let tx = tx.clone();
        let handler = move |request: JsonRpcRequest| {
            let tx = tx.clone();
            async move {
                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                let _ = tx.send(IpcCommand::Request(request, response_tx));
                response_rx
                    .await
                    .unwrap_or_else(|_| rpc_error(-32000, "Internal error", None::<String>, 1))
            }
        };

        let ipc_server = IpcServer::new();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                if let Err(e) = ipc_server.run(handler).await {
                    tracing::error!("IPC server error: {}", e);
                }
            });
        });
    }

    info!("Waio Daemon running. Waiting for auras...");
    info!("Use waio-cli to load auras");

    // 6. Main dispatch loop.
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_for_handler = shutdown_flag.clone();

    loop {
        // Dispatch Wayland events (configure, frame, closed, etc.).
        match event_loop.dispatch(Duration::from_millis(16), &mut wl_state) {
            Ok(_) => {}
            Err(e) => {
                let err_str = e.to_string();
                tracing::warn!("Event loop dispatch error: {:?}", e);
                if err_str.contains("Broken pipe") || err_str.contains("Connection closed") {
                    tracing::info!("Wayland connection closed, exiting gracefully");
                    break;
                }
                break;
            }
        }

        // Check if shutdown was requested.
        if shutdown_flag.load(Ordering::Relaxed) {
            info!("Shutdown flag set, exiting event loop");
            break;
        }

        // Process Lua property updates and redraw dirty surfaces (replaces timer).
        if let Err(e) = renderer_for_ipc.process_commands() {
            tracing::warn!("Command processing error: {}", e);
        }
        // Process timer fires — calls Lua callbacks in the main thread.
        renderer_for_ipc.process_timer_fires();
        if let Err(e) = renderer_for_ipc.redraw_all() {
            tracing::warn!("Redraw error: {}", e);
        }

        // Process IPC commands (load/unload auras).
        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                IpcCommand::Request(request, response_tx) => {
                    debug!("Processing IPC request: {:?}", request.method);
                    let response = handle_request(
                        request,
                        auras.clone(),
                        renderer_for_ipc.clone(),
                        &mut wl_state,
                        shutdown_for_handler.clone(),
                    );
                    debug!("IPC request completed");
                    let _ = response_tx.send(response);
                }
            }
        }
    }

    // Cleanup on exit.
    if let Err(e) = renderer.shutdown() {
        tracing::warn!("Shutdown error: {}", e);
    }

    Ok(())
}

/// Handle a single IPC request.
fn handle_request(
    request: JsonRpcRequest,
    auras: Arc<Mutex<HashMap<String, Aura>>>,
    renderer: Rc<SlintRenderer>,
    wl_state: &mut crate::infrastructure::wayland::WlState,
    shutdown_flag: Arc<AtomicBool>,
) -> JsonRpcResponse {
    let method: Result<DaemonMethod, _> = serde_json::from_value(request.params.clone());

    match method {
        Ok(DaemonMethod::LoadAura {
            source,
            path,
            content,
            id,
        }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None::<String>, request.id)
                }
            };

            if auras.contains_key(&id) {
                return rpc_error(
                    -32002,
                    "Aura with this ID already loaded",
                    None::<String>,
                    request.id,
                );
            }

            let aura_file = match source.as_str() {
                "file" => {
                    let path = match path {
                        Some(p) => p,
                        None => {
                            return rpc_error(
                                -32602,
                                "path required",
                                None::<String>,
                                request.id,
                            )
                        }
                    };
                    AuraFile::from_path(std::path::Path::new(&path))
                }
                "inline" => {
                    let content = match content {
                        Some(c) => c,
                        None => {
                            return rpc_error(
                                -32602,
                                "content required",
                                None::<String>,
                                request.id,
                            )
                        }
                    };
                    AuraFile::from_content(&content)
                }
                _ => {
                    return rpc_error(-32602, "Invalid source", None::<String>, request.id)
                }
            };

            match aura_file {
                Ok(aura_file) => {
                    let aura = aura_file.to_aura();

                    // Key change: load_aura creates the surface (pending state) and runs Lua.
                    // The first render happens later when the compositor sends configure.
                    match renderer.load_aura(&aura, &id, wl_state) {
                        Ok(_) => {
                            auras.insert(id.clone(), aura);
                            rpc_success(
                                serde_json::json!({ "status": "ok", "id": id }),
                                request.id,
                            )
                        }
                        Err(e) => {
                            rpc_error(-32000, &e.to_string(), None::<String>, request.id)
                        }
                    }
                }
                Err(e) => rpc_error(-32001, &e.to_string(), None::<String>, request.id),
            }
        }

        Ok(DaemonMethod::UnloadAura { id }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None::<String>, request.id)
                }
            };

            if !auras.contains_key(&id) {
                return rpc_error(-32001, "Aura not found", None::<String>, request.id);
            }

            match renderer.remove_aura(&id) {
                Ok(_) => {
                    auras.remove(&id);
                    rpc_success(serde_json::json!({ "status": "ok" }), request.id)
                }
                Err(e) => rpc_error(-32000, &e.to_string(), None::<String>, request.id),
            }
        }

        Ok(DaemonMethod::ListAuras) => {
            let auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None, request.id)
                }
            };

            let list: Vec<serde_json::Value> = auras
                .iter()
                .map(|(id, aura)| {
                    serde_json::json!({
                        "id": id,
                        "name": aura.name,
                        "type": format!("{:?}", aura.aura_type)
                    })
                })
                .collect();

            rpc_success(serde_json::json!({ "auras": list }), request.id)
        }

        Ok(DaemonMethod::SystemInfo) => {
            let auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None, request.id)
                }
            };

            rpc_success(
                serde_json::json!({
                    "version": "0.1.0",
                    "name": "Waio Daemon",
                    "auras_count": auras.len()
                }),
                request.id,
            )
        }

        Ok(DaemonMethod::SystemShutdown) => {
            info!("Shutdown requested via IPC");
            shutdown_flag.store(true, Ordering::Relaxed);
            rpc_success(
                serde_json::json!({ "status": "ok", "message": "Shutting down" }),
                request.id,
            )
        }

        Ok(DaemonMethod::UpdateAura { id, content }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None, request.id)
                }
            };

            if !auras.contains_key(&id) {
                return rpc_error(-32001, "Aura not found", None::<String>, request.id);
            }

            match AuraFile::from_content(&content) {
                Ok(aura_file) => {
                    let aura = aura_file.to_aura();
                    let _ = renderer.remove_aura(&id);
                    match renderer.load_aura(&aura, &id, wl_state) {
                        Ok(_) => {
                            auras.insert(id.clone(), aura);
                            rpc_success(
                                serde_json::json!({ "status": "ok", "id": id }),
                                request.id,
                            )
                        }
                        Err(e) => {
                            rpc_error(-32000, &e.to_string(), None::<String>, request.id)
                        }
                    }
                }
                Err(e) => rpc_error(-32001, &e.to_string(), None::<String>, request.id),
            }
        }

        Err(e) => rpc_error(-32602, &e.to_string(), None, request.id),
    }
}

fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,waio_daemon=debug")),
        )
        .try_init();
}
