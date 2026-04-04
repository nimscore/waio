pub mod time;
pub mod timer;
pub mod slint_bridge;

use crate::infrastructure::lua::timer::TimerRegistry;
use mlua::prelude::*;

pub fn register_all(lua: &Lua, timer_registry: TimerRegistry) -> LuaResult<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    waio.set("timer", timer::create_module(lua, timer_registry)?)?;
    lua.globals().set("waio", waio)?;
    Ok(())
}
