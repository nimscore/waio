use chrono::{Datelike, Local, Timelike};
use mlua::prelude::*;

pub fn create_module(lua: &Lua) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    m.set(
        "now",
        lua.create_function(|lua, ()| {
            let now = Local::now();
            let r = lua.create_table()?;
            r.set("year", now.year())?;
            r.set("month", now.month())?;
            r.set("day", now.day())?;
            r.set("hour", now.hour())?;
            r.set("min", now.minute())?;
            r.set("sec", now.second())?;
            r.set("str", now.format("%H:%M:%S").to_string())?;
            Ok(r)
        })?,
    )?;

    m.set(
        "format",
        lua.create_function(|_, fmt: String| Ok(Local::now().format(&fmt).to_string()))?,
    )?;

    m.set(
        "unix",
        lua.create_function(|_, ()| Ok(Local::now().timestamp()))?,
    )?;

    Ok(m)
}
