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

/// Register `slint` in the given restricted environment table AND in globals.
/// The callback executes in globals context, so slint must be available there too.
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

    // Set in BOTH restricted env AND globals (callbacks run in globals context).
    env.set("slint", &slint_table)?;
    lua.globals().set("slint", slint_table)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_property() {
        update_property_store("aura-1", "TimeLayer.time_text", "12:00:00");
        let val = get_property_from_store("aura-1", "TimeLayer.time_text");
        assert_eq!(val, Some("12:00:00".to_string()));
    }

    #[test]
    fn test_clear_property_store() {
        update_property_store("aura-1", "prop", "value");
        clear_property_store("aura-1");
        let val = get_property_from_store("aura-1", "prop");
        assert_eq!(val, None);
    }

    #[test]
    fn test_aura_isolation() {
        update_property_store("aura-1", "shared_prop", "value1");
        update_property_store("aura-2", "shared_prop", "value2");
        assert_eq!(get_property_from_store("aura-1", "shared_prop"), Some("value1".to_string()));
        assert_eq!(get_property_from_store("aura-2", "shared_prop"), Some("value2".to_string()));
    }

    #[test]
    fn test_missing_property_returns_none() {
        assert_eq!(get_property_from_store("nonexistent", "prop"), None);
    }
}
