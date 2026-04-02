use mlua::prelude::*;
use std::time::Duration;

pub fn create_module(lua: &Lua) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    m.set(
        "interval",
        lua.create_function(|_, (ms, cb): (u32, LuaFunction)| {
            let cb_clone = cb.clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_millis(ms as u64));
                let _ = cb_clone.call::<()>(());
            });
            Ok(())
        })?,
    )?;

    m.set(
        "timeout",
        lua.create_function(|_, (ms, cb): (u32, LuaFunction)| {
            let cb_clone = cb.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(ms as u64));
                let _ = cb_clone.call::<()>(());
            });
            Ok(())
        })?,
    )?;

    Ok(m)
}
