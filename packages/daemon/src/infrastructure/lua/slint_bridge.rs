use crate::infrastructure::slint::aura_handle::{push_command, CommandQueue, PropertyUpdate};
use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

/// Per-aura property store for Lua `slint.get_property()`.
/// Populated by `process_commands()` when properties are set.
static PROPERTY_STORE: LazyLock<RwLock<HashMap<String, HashMap<String, String>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Update the property store when a property is set.
pub fn update_property_store(aura_id: &str, property: &str, value: &str) {
    let mut store = PROPERTY_STORE.write().unwrap_or_else(|e| e.into_inner());
    let aura_props = store.entry(aura_id.to_string()).or_default();
    aura_props.insert(property.to_string(), value.to_string());
}

/// Clear all properties for an aura (called on unload).
pub fn clear_property_store(aura_id: &str) {
    let mut store = PROPERTY_STORE.write().unwrap_or_else(|e| e.into_inner());
    store.remove(aura_id);
}

/// Get a property value for an aura.
pub fn get_property_from_store(aura_id: &str, property: &str) -> Option<String> {
    let store = PROPERTY_STORE.read().unwrap_or_else(|e| e.into_inner());
    store.get(aura_id).and_then(|props| props.get(property).cloned())
}

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

/// Register `slint.set_property` and `slint.get_property` in the given environment table.
pub fn register_with_queue(lua: &Lua, aura_id: String, queue: CommandQueue) -> LuaResult<()> {
    let slint_table = lua.create_table()?;

    let id_for_set = aura_id.clone();
    let queue_for_set = queue.clone();

    slint_table.set(
        "set_property",
        lua.create_function(move |_, (name, value): (String, String)| {
            push_command(&queue_for_set, PropertyUpdate {
                aura_id: id_for_set.clone(),
                property: name.clone(),
                value: value.clone(),
            });
            update_property_store(&id_for_set, &name, &value);
            Ok(())
        })?,
    )?;

    let id_for_get = aura_id.clone();
    slint_table.set(
        "get_property",
        lua.create_function(move |_, name: String| -> LuaResult<String> {
            Ok(get_property_from_store(&id_for_get, &name).unwrap_or_default())
        })?,
    )?;

    // Set in globals — caller should also copy to restricted env.
    lua.globals().set("slint", slint_table)?;
    Ok(())
}

/// Register `slint` in the given restricted environment table.
/// Call this after `register_with_queue` to make `slint` visible in the sandbox.
pub fn register_slint_in_env(lua: &Lua, env: &LuaTable, aura_id: String, queue: CommandQueue) -> LuaResult<()> {
    let slint_table = lua.create_table()?;

    let id_for_set = aura_id.clone();
    let queue_for_set = queue.clone();

    slint_table.set(
        "set_property",
        lua.create_function(move |_, (name, value): (String, String)| {
            push_command(&queue_for_set, PropertyUpdate {
                aura_id: id_for_set.clone(),
                property: name.clone(),
                value: value.clone(),
            });
            update_property_store(&id_for_set, &name, &value);
            Ok(())
        })?,
    )?;

    let id_for_get = aura_id.clone();
    slint_table.set(
        "get_property",
        lua.create_function(move |_, name: String| -> LuaResult<String> {
            Ok(get_property_from_store(&id_for_get, &name).unwrap_or_default())
        })?,
    )?;

    env.set("slint", slint_table)?;
    Ok(())
}
