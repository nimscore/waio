use crate::infrastructure::slint::SlintRenderer;
use crate::infrastructure::wayland::WaylandConnection;
use anyhow::Result;
use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use std::rc::Rc;
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

    // Render timer: process commands + redraw
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

    // Initial dispatch
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

    renderer.render_aura_with_state(&test_aura, &mut wl_state)?;

    // Dispatch for configure
    event_loop
        .dispatch(Duration::from_millis(100), &mut wl_state)
        .expect("Configure dispatch failed");

    info!("Waio Daemon running. Press Ctrl+C to stop.");

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
