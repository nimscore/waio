pub mod sandbox;
pub mod time;
pub mod timer;
pub mod slint_bridge;

use mlua::prelude::*;

/// Create a sandboxed Lua state with dangerous functions removed.
pub fn create_sandboxed_lua() -> mlua::Lua {
    let lua = mlua::Lua::new_with(
        mlua::StdLib::ALL_SAFE,
        mlua::LuaOptions::new().catch_rust_panics(true),
    ).expect("Failed to create sandboxed Lua state");

    sandbox::sanitize_globals(&lua).expect("Failed to sanitize Lua globals");

    // Register waio.* modules (safe subset).
    register_all(&lua).expect("Failed to register waio modules");

    lua
}

/// Register common `waio.*` modules (time, etc.) WITHOUT timer.
/// Timer is registered per-aura via `timer::create_module`.
pub fn register_all(lua: &Lua) -> LuaResult<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    lua.globals().set("waio", waio)?;
    Ok(())
}
