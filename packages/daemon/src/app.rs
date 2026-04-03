use crate::controller::IpcServer;
use crate::infrastructure::slint::SlintRenderer;
use crate::infrastructure::wayland::WaylandConnection;
use crate::usecase::render::Renderer;
use anyhow::Result;
use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::EnvFilter;
use waio_shared::entity::{Aura, AuraFile};
use waio_shared::protocol::{DaemonMethod, JsonRpcRequest, JsonRpcResponse, rpc_success, rpc_error};

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

pub fn run() -> Result<()> {
    info!("Starting Waio Daemon...");
    init_logging();

    if std::env::var("WAYLAND_DISPLAY").is_err() {
        anyhow::bail!(
            "WAYLAND_DISPLAY not set. Try: WAYLAND_DISPLAY=wayland-0 cargo run -p waio-daemon"
        );
    }

    let (wl_conn, event_queue, mut wl_state) = WaylandConnection::connect()?;

    let renderer = Rc::new(SlintRenderer::new(
        wl_conn.compositor.clone(),
        wl_conn.qh.clone(),
    ));

    let mut event_loop: EventLoop<crate::infrastructure::wayland::WlState> =
        EventLoop::try_new().expect("Failed to create event loop");

    WaylandSource::new(wl_conn.conn.clone(), event_queue)
        .insert(event_loop.handle())
        .expect("Failed to insert WaylandSource");

    let renderer_for_timer = renderer.clone();
    event_loop
        .handle()
        .insert_source(
            smithay_client_toolkit::reexports::calloop::timer::Timer::from_duration(
                Duration::from_millis(16),
            ),
            move |_, _, _| {
                let _ = renderer_for_timer.process_commands();
                let _ = renderer_for_timer.redraw_all();
                smithay_client_toolkit::reexports::calloop::timer::TimeoutAction::ToDuration(
                    Duration::from_millis(16),
                )
            },
        )
        .expect("Failed to insert timer");

    event_loop
        .dispatch(Duration::from_millis(100), &mut wl_state)
        .expect("Initial dispatch failed");

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
                response_rx.await.unwrap_or_else(|_| {
                    rpc_error(-32000, "Internal error", None::<String>, 1)
                })
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

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut wl_state)
            .expect("Event loop dispatch failed");

        while let Ok(cmd) = rx.try_recv() {
            match cmd {
                IpcCommand::Request(request, response_tx) => {
                    let response = handle_request(
                        request,
                        auras.clone(),
                        renderer_for_ipc.clone(),
                        &mut wl_state,
                    );
                    let _ = response_tx.send(response);
                }
            }
        }
    }
}

fn handle_request(
    request: JsonRpcRequest,
    auras: Arc<Mutex<HashMap<String, Aura>>>,
    renderer: Rc<SlintRenderer>,
    wl_state: &mut crate::infrastructure::wayland::WlState,
) -> JsonRpcResponse {
    let method: Result<DaemonMethod, _> = serde_json::from_value(request.params.clone());

    match method {
        Ok(DaemonMethod::LoadAura { source, path, content, id }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => return rpc_error(-32000, &e.to_string(), None, request.id),
            };

            if auras.contains_key(&id) {
                return rpc_error(-32002, "Aura with this ID already loaded", None::<String>, request.id);
            }

            let aura_file = match source.as_str() {
                "file" => {
                    let path = match path {
                        Some(p) => p,
                        None => return rpc_error(-32602, "path required", None::<String>, request.id),
                    };
                    waio_shared::entity::AuraFile::from_path(std::path::Path::new(&path))
                }
                "inline" => {
                    let content = match content {
                        Some(c) => c,
                        None => return rpc_error(-32602, "content required", None::<String>, request.id),
                    };
                    waio_shared::entity::AuraFile::from_content(&content)
                }
                _ => return rpc_error(-32602, "Invalid source", None::<String>, request.id),
            };

            match aura_file {
                Ok(aura_file) => {
                    let aura = aura_file.to_aura();
                    
                    match renderer.render_aura_with_state(&aura, wl_state) {
                        Ok(_) => {
                            auras.insert(id.clone(), aura);
                            rpc_success(serde_json::json!({ "status": "ok", "id": id }), request.id)
                        }
                        Err(e) => rpc_error(-32000, &e.to_string(), None::<String>, request.id),
                    }
                }
                Err(e) => rpc_error(-32001, &e.to_string(), None::<String>, request.id),
            }
        }
        Ok(DaemonMethod::UnloadAura { id }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => return rpc_error(-32000, &e.to_string(), None::<String>, request.id),
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
                Err(e) => return rpc_error(-32000, &e.to_string(), None, request.id),
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
                Err(e) => return rpc_error(-32000, &e.to_string(), None, request.id),
            };

            rpc_success(serde_json::json!({
                "version": "0.1.0",
                "name": "Waio Daemon",
                "auras_count": auras.len()
            }), request.id)
        }
        Ok(DaemonMethod::SystemShutdown) => {
            info!("Shutdown requested via IPC");
            rpc_success(serde_json::json!({ "status": "ok", "message": "Shutting down" }), request.id)
        }
        Ok(DaemonMethod::UpdateAura { id, content }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => return rpc_error(-32000, &e.to_string(), None, request.id),
            };

            if !auras.contains_key(&id) {
                return rpc_error(-32001, "Aura not found", None::<String>, request.id);
            }

            match AuraFile::from_content(&content) {
                Ok(aura_file) => {
                    let aura = aura_file.to_aura();
                    match renderer.render_aura_with_state(&aura, wl_state) {
                        Ok(_) => {
                            auras.insert(id.clone(), aura);
                            rpc_success(serde_json::json!({ "status": "ok", "id": id }), request.id)
                        }
                        Err(e) => rpc_error(-32000, &e.to_string(), None::<String>, request.id),
                    }
                }
                Err(e) => rpc_error(-32001, &e.to_string(), None, request.id),
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