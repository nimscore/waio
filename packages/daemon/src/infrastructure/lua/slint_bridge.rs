#![allow(dead_code)]

use crate::infrastructure::slint::aura_handle::{CommandQueue, PropertyUpdate};
use mlua::prelude::*;

pub fn register(lua: &Lua) -> LuaResult<()> {
    let slint_table = lua.create_table()?;
    slint_table.set(
        "set_property",
        lua.create_function(|_, (name, value): (String, String)| {
            tracing::info!("slint.set_property({}, {}) - no queue", name, value);
            Ok(())
        })?,
    )?;
    slint_table.set(
        "get_property",
        lua.create_function(|_, _name: String| Ok(String::new()))?,
    )?;
    lua.globals().set("slint", slint_table)?;
    Ok(())
}

pub fn register_with_queue(lua: &Lua, aura_id: String, queue: CommandQueue) -> LuaResult<()> {
    let slint_table = lua.create_table()?;

    let id_for_set = aura_id.clone();
    let queue_for_set = queue.clone();

    slint_table.set(
        "set_property",
        lua.create_function(move |_, (name, value): (String, String)| {
            let mut q = queue_for_set
                .lock()
                .map_err(|e| mlua::Error::RuntimeError(format!("queue lock: {}", e)))?;
            q.push(PropertyUpdate {
                aura_id: id_for_set.clone(),
                property: name,
                value,
            });
            Ok(())
        })?,
    )?;

    slint_table.set(
        "get_property",
        lua.create_function(|_, _name: String| Ok(String::new()))?,
    )?;

    lua.globals().set("slint", slint_table)?;
    Ok(())
}
