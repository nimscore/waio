use crate::infrastructure::slint::SlintRenderer;
use crate::infrastructure::wayland::WaylandConnection;
use anyhow::Result;
use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub fn run() -> Result<()> {
    info!("Starting Waio Daemon...");
    init_logging();

    if std::env::var("WAYLAND_DISPLAY").is_err() {
        anyhow::bail!(
            "WAYLAND_DISPLAY not set. Try: WAYLAND_DISPLAY=wayland-0 cargo run -p waio-daemon"
        );
    }

    // Connect to Wayland - returns connection, event_queue, and state
    let (wl_conn, event_queue, mut wl_state) = WaylandConnection::connect()?;

    let renderer = SlintRenderer::new(wl_conn.compositor.clone(), wl_conn.qh.clone());

    // Setup calloop event loop with the event_queue from connection
    let mut event_loop: EventLoop<crate::infrastructure::wayland::WlState> =
        EventLoop::try_new().expect("Failed to create event loop");

    WaylandSource::new(wl_conn.conn.clone(), event_queue)
        .insert(event_loop.handle())
        .expect("Failed to insert WaylandSource");

    // Dispatch once to process initial globals
    event_loop
        .dispatch(Duration::from_millis(100), &mut wl_state)
        .expect("Initial dispatch failed");

    // Create aura
    let lua_code = r#"
        waio.timer.interval(1000, function()
            local time = waio.time.now()
            local date = waio.time.format("%Y-%m-%d")
            slint.set_property("time_text", time.str)
            slint.set_property("date_text", date)
        end)
        print("Clock aura started!")
    "#;

    let mut test_aura = crate::usecase::aura::create_aura_without_repo(
        "test-bar".into(),
        waio_shared::entity::Aura::default().slint_code,
        waio_shared::entity::AuraType::Bar,
    )?;
    test_aura.lua_code = Some(lua_code.to_string());

    info!("Created test aura: {}", test_aura.id);

    // Step 1: Create surface (initial commit without buffer)
    let mut surface = renderer.create_surface(&test_aura, &mut wl_state)?;

    // Step 2: Dispatch to handle configure
    event_loop
        .dispatch(Duration::from_millis(100), &mut wl_state)
        .expect("Configure dispatch failed");

    // Step 3: Now draw on the surface
    renderer.draw_aura(&test_aura, &mut surface)?;

    // Store surface so it doesn't get dropped
    // For MVP we'll just forget it
    std::mem::forget(surface);

    info!("Waio Daemon running. Press Ctrl+C to stop.");

    // Main event loop
    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut wl_state)
            .expect("Event loop dispatch failed");
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
