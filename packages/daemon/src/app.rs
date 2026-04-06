use crate::controller::IpcServer;
use crate::infrastructure::repository::FileAuraRepository;
use crate::infrastructure::slint::SlintRenderer;
use crate::infrastructure::wayland::WaylandConnection;
use crate::usecase::aura::AuraRepository;
use crate::usecase::render::Renderer;
use anyhow::Result;
use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::time::Duration;
use tracing::{debug, info, warn};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;
use waio_shared::config::WaioConfig;
use waio_shared::entity::{Aura, AuraFile};
use waio_shared::protocol::{
    DaemonMethod, JsonRpcRequest, JsonRpcResponse, error_codes, rpc_error, rpc_success,
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
    // Load configuration: WAIO_CONFIG env → ~/.config/waio/config.yaml → create default
    let config = WaioConfig::load()
        .map_err(|e| anyhow::anyhow!("Failed to load config: {}", e))?;
    config.validate();

    init_logging(&config.log_level());

    info!("Config loaded: socket={}, data_dir={}", config.socket_path(), config.data_dir());

    if std::env::var("WAYLAND_DISPLAY").is_err() {
        anyhow::bail!(
            "WAYLAND_DISPLAY not set. Try: WAYLAND_DISPLAY=wayland-0 cargo run -p waio-daemon"
        );
    }

    // 1. Connect to Wayland and get all the necessary state objects.
    let (wl_conn, event_queue, mut wl_state) = WaylandConnection::connect()?;

    // 2. Create the Slint renderer — it owns compositor and qh.
    //    LayerShell and Shm remain in WlState and are borrowed during load_aura.
    let (renderer, timer_channel) = SlintRenderer::new(
        wl_state.compositor_state.clone(),
        wl_conn.qh.clone(),
    );
    let renderer = Rc::new(renderer);

    // 3. Give the renderer to WlState so handlers can dispatch events to it.
    wl_state.renderer = Some(renderer.clone());

    // 4. Create the aura repository for persistence.
    let data_dir = PathBuf::from(config.data_dir());
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        warn!("Failed to create data_dir {}: {}", data_dir.display(), e);
    }
    let repo = Arc::new(FileAuraRepository::new(data_dir.clone()));
    info!("Aura repository initialized at: {}", data_dir.display());

    // 5. Create the calloop event loop and insert WaylandSource into it.
    let mut event_loop: EventLoop<crate::infrastructure::wayland::WlState> =
        EventLoop::try_new().expect("Failed to create event loop");

    WaylandSource::new(wl_conn.conn.clone(), event_queue)
        .insert(event_loop.handle())
        .expect("Failed to insert WaylandSource");

    // 5. Set up IPC server on a separate thread.
    let state = AppState::new();
    let auras = state.auras.clone();
    let renderer_for_ipc = renderer.clone();
    let repo_for_ipc = repo.clone();

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

        let ipc_server = IpcServer::with_socket_path(Some(PathBuf::from(config.socket_path())));
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

    // 5b. Insert the timer channel into the calloop event loop.
    // Timer fires from Lua threads are delivered as calloop events, invoking
    // the callback in the main thread (safe for Lua API calls).
    {
        let renderer_for_timer = renderer.clone();
        event_loop
            .handle()
            .insert_source(
                timer_channel,
                move |event, (), _shared_data| {
                    if let smithay_client_toolkit::reexports::calloop::channel::Event::Msg(fire) = event {
                        renderer_for_timer.process_single_timer_fire(fire);
                    }
                },
            )
            .expect("Failed to insert timer channel into event loop");
        info!("Timer channel inserted into event loop");
    }

    info!("Waio Daemon running. Waiting for auras...");
    info!("Use waio-cli to load auras");

    // 6. Recovery: restore previously loaded auras from the repository.
    match repo.list() {
        Ok(saved_auras) => {
            for aura in saved_auras {
                let aura_id = aura.id.clone();
                if auras.lock().map(|a| a.contains_key(&aura_id)).unwrap_or(false) {
                    warn!("Aura {} already in memory, skipping", aura_id);
                    continue;
                }
                match renderer.load_aura(&aura, &aura_id, &mut wl_state) {
                    Ok(_) => {
                        let aura_name = aura.name.clone();
                        if let Ok(mut a) = auras.lock() {
                            a.insert(aura_id.clone(), aura);
                        }
                        info!("Restored aura: {} ({})", aura_id, aura_name);
                    }
                    Err(e) => {
                        warn!("Failed to restore aura {}: {}", aura_id, e);
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to list saved auras: {}", e);
        }
    }

    // 6. Single instance: create PID file with flock.
    let state_dir = dirs::state_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("waio");
    let _ = std::fs::create_dir_all(&state_dir);
    let pid_path = state_dir.join("waio-daemon.pid");
    let pid_file = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&pid_path)
    {
        Ok(f) => f,
        Err(e) => {
            anyhow::bail!("Failed to open PID file {}: {}", pid_path.display(), e);
        }
    };
    // Try to acquire exclusive lock (non-blocking).
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let ret = unsafe { libc::flock(pid_file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
        if ret != 0 {
            // Another instance holds the lock.
            if let Ok(content) = std::fs::read_to_string(&pid_path) {
                anyhow::bail!(
                    "Another waio-daemon instance is running (PID: {}). \
                     If this is stale, remove {} and try again.",
                    content.trim(),
                    pid_path.display()
                );
            } else {
                anyhow::bail!(
                    "Another waio-daemon instance is running. Remove {} and try again.",
                    pid_path.display()
                );
            }
        }
    }
    // Write our PID.
    let our_pid = std::process::id();
    let _ = std::fs::write(&pid_path, our_pid.to_string());
    info!("PID file created: {} (PID {})", pid_path.display(), our_pid);

    // 7. Main dispatch loop.
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_for_handler = shutdown_flag.clone();

    // Set up signal handlers: SIGTERM (systemd) + SIGINT (Ctrl+C).
    let shutdown_for_sig = shutdown_flag.clone();
    ctrlc::set_handler(move || {
        info!("Signal received, initiating graceful shutdown...");
        shutdown_for_sig.store(true, Ordering::Relaxed);
    }).expect("Failed to set signal handler");

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

        // Process Lua property updates and redraw dirty surfaces.
        if let Err(e) = renderer_for_ipc.process_commands() {
            tracing::warn!("Command processing error: {}", e);
        }
        // Timer fires are handled by calloop channel callback (inserted above).
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
                        repo_for_ipc.clone(),
                        &mut wl_state,
                        shutdown_for_handler.clone(),
                    );
                    debug!("IPC request completed");
                    let _ = response_tx.send(response);
                }
            }
        }
    }

    // Graceful shutdown: close surfaces, cancel timers, clean up resources.
    info!("Starting graceful shutdown...");
    let aura_count = auras.lock().map(|a| a.len()).unwrap_or(0);
    info!("Shutting down {} aura(s)...", aura_count);
    if let Err(e) = renderer.shutdown() {
        tracing::warn!("Shutdown error: {}", e);
    }

    // Remove PID file.
    let _ = std::fs::remove_file(&pid_path);
    // PID file lock is released automatically when pid_file is dropped.
    info!("PID file removed: {}", pid_path.display());

    // Remove stale socket file.
    let socket_path = std::path::PathBuf::from(config.socket_path());
    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
        info!("Socket file removed: {}", socket_path.display());
    }

    info!("Waio Daemon shutdown complete");

    Ok(())
}

/// Handle a single IPC request.
fn handle_request(
    request: JsonRpcRequest,
    auras: Arc<Mutex<HashMap<String, Aura>>>,
    renderer: Rc<SlintRenderer>,
    repo: Arc<FileAuraRepository>,
    wl_state: &mut crate::infrastructure::wayland::WlState,
    shutdown_flag: Arc<AtomicBool>,
) -> JsonRpcResponse {
    let method: Result<DaemonMethod, _> = serde_json::from_value(request.params.clone());

    match method {
        Ok(DaemonMethod::LoadAura {
            source,
            path,
            content,
            id: requested_id,
        }) => {
            let mut auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None::<String>, request.id)
                }
            };

            let aura_file = match source.as_str() {
                "file" => {
                    let path = match &path {
                        Some(p) => p.clone(),
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
                    // Canonical ID: from YAML meta.id (or requested_id if provided).
                    let aura_id = requested_id.unwrap_or_else(|| aura.id.clone());

                    if auras.contains_key(&aura_id) {
                        return rpc_error(
                            error_codes::ALREADY_EXISTS,
                            &format!("Aura '{}' already loaded", aura_id),
                            None::<String>,
                            request.id,
                        );
                    }

                    match renderer.load_aura(&aura, &aura_id, wl_state) {
                        Ok(_) => {
                            // Persist to disk.
                            if let Err(e) = repo.save(&aura) {
                                warn!("Failed to persist aura {}: {}", aura_id, e);
                            }
                            auras.insert(aura_id.clone(), aura);
                            rpc_success(
                                serde_json::json!({ "status": "ok", "id": aura_id }),
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
                return rpc_error(error_codes::NOT_FOUND, "Aura not found", None::<String>, request.id);
            }

            match renderer.remove_aura(&id) {
                Ok(_) => {
                    // Remove from persistence.
                    if let Err(e) = repo.delete(&id) {
                        warn!("Failed to remove persisted aura {}: {}", id, e);
                    }
                    auras.remove(&id);
                    rpc_success(serde_json::json!({ "status": "ok" }), request.id)
                }
                Err(e) => rpc_error(-32000, &e.to_string(), None::<String>, request.id),
            }
        }

        Ok(DaemonMethod::SystemStatus) => {
            let auras = match auras.lock() {
                Ok(a) => a,
                Err(e) => {
                    return rpc_error(-32000, &e.to_string(), None, request.id)
                }
            };

            let auras_list: Vec<serde_json::Value> = auras
                .iter()
                .map(|(id, aura)| {
                    serde_json::json!({
                        "id": id,
                        "name": aura.name,
                        "type": format!("{:?}", aura.aura_type)
                    })
                })
                .collect();

            rpc_success(
                serde_json::json!({
                    "status": "running",
                    "version": "0.1.0",
                    "protocol_version": waio_shared::protocol::PROTOCOL_VERSION,
                    "auras": auras_list
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
                            // Persist updated aura.
                            if let Err(e) = repo.save(&aura) {
                                warn!("Failed to persist updated aura {}: {}", id, e);
                            }
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

fn init_logging(log_level: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    // Try journald first; fall back to rolling file appender.
    if let Ok(layer) = tracing_journald::layer() {
        let _ = tracing_subscriber::registry()
            .with(layer.with_filter(env_filter))
            .try_init();
        eprintln!("Logging to journald");
        info!("Starting Waio Daemon...");
        return;
    }

    // Fallback: rolling file appender.
    let log_dir = dirs::state_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("waio/logs");
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(&log_dir, "waio-daemon.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(non_blocking)
        .try_init();

    info!("Starting Waio Daemon...");
}
