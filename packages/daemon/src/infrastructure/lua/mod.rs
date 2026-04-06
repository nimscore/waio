pub mod audio;
pub mod backlight;
pub mod bluetooth;
pub mod clipboard;
pub mod exec;
pub mod fs;
pub mod http;
pub mod input;
pub mod notify;
pub mod power;
pub mod rate_limiter;
pub mod sandbox;
pub mod slint_bridge;
pub mod sys;
pub mod time;
pub mod timer;
pub mod wifi;

pub use rate_limiter::RateLimiter;

use mlua::prelude::*;

/// Create a sandboxed Lua state with dangerous functions removed.
pub fn create_sandboxed_lua() -> mlua::Lua {
    let lua = mlua::Lua::new_with(
        mlua::StdLib::ALL_SAFE,
        mlua::LuaOptions::new().catch_rust_panics(true),
    )
    .expect("Failed to create sandboxed Lua state");

    sandbox::sanitize_globals(&lua).expect("Failed to sanitize Lua globals");

    // Register waio.* modules (safe subset).
    // Rate limiter is not available yet; modules registered here (time, sys, clipboard)
    // do not require rate limiting.
    register_all(&lua, None).expect("Failed to register waio modules");

    lua
}

/// Register common `waio.*` modules (time, etc.) WITHOUT timer.
/// Timer is registered per-aura via `timer::create_module`.
pub fn register_all(lua: &Lua, _rate_limiter: Option<RateLimiter>) -> LuaResult<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    waio.set("sys", sys::create_module(lua)?)?;
    waio.set("clipboard", clipboard::create_module(lua)?)?;
    lua.globals().set("waio", waio)?;
    Ok(())
}
