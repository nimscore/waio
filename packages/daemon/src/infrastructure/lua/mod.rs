pub mod time;
pub mod timer;
pub mod slint_bridge;

use crate::infrastructure::lua::timer::TimerRegistry;
use mlua::prelude::*;

/// Register common `waio.*` modules (time, etc.) WITHOUT timer.
/// Timer is registered per-aura via `register_timer_for_aura`.
pub fn register_all(lua: &Lua) -> LuaResult<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    // Timer is NOT registered here — it needs aura_id.
    lua.globals().set("waio", waio)?;
    Ok(())
}

/// Register `waio.timer` module for a specific aura.
/// All timers created from this Lua context will be associated with `aura_id`.
pub fn register_timer_for_aura(
    lua: &Lua,
    timer_registry: TimerRegistry,
    aura_id: String,
) -> LuaResult<()> {
    let waio: LuaTable = lua.globals().get("waio")?;
    waio.set("timer", timer::create_module(lua, timer_registry, aura_id)?)?;
    Ok(())
}
