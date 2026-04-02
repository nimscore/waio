pub mod time;
pub mod timer;
pub mod slint_bridge;

use mlua::prelude::*;

pub fn register_all(lua: &Lua) -> LuaResult<()> {
    let waio = lua.create_table()?;
    waio.set("time", time::create_module(lua)?)?;
    waio.set("timer", timer::create_module(lua)?)?;
    lua.globals().set("waio", waio)?;
    Ok(())
}
