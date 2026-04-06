use mlua::prelude::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tracing::{debug, warn};

/// Cursor types supported by `waio.input.set_cursor`.
#[derive(Debug, Clone, Default)]
pub enum CursorType {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Crosshair,
    Wait,
}

impl CursorType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "pointer" => CursorType::Pointer,
            "text" => CursorType::Text,
            "grab" => CursorType::Grab,
            "crosshair" => CursorType::Crosshair,
            "wait" => CursorType::Wait,
            _ => CursorType::Default,
        }
    }
}

/// Get the current keyboard layout using `setxkbmap -query`.
///
/// Returns the first layout from the XKB configuration (e.g., "en", "ru", "us").
/// Returns "unknown" if `setxkbmap` is not available or parsing fails.
pub fn get_keyboard_layout() -> String {
    let output = match Command::new("setxkbmap").arg("-query").output() {
        Ok(o) => o,
        Err(e) => {
            debug!("setxkbmap not available: {}", e);
            return "unknown".to_string();
        }
    };

    if !output.status.success() {
        debug!("setxkbmap -query failed");
        return "unknown".to_string();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if let Some(layouts) = line.strip_prefix("layout:") {
            let layouts = layouts.trim();
            // Return the first layout if multiple are present (comma-separated).
            return layouts
                .split(',')
                .next()
                .unwrap_or("unknown")
                .trim()
                .to_string();
        }
    }

    debug!("Could not parse layout from setxkbmap output");
    "unknown".to_string()
}

/// Per-aura input callback storage.
///
/// Holds Lua callbacks registered via `waio.input.on_*` functions.
/// The renderer calls `trigger_*` methods when pointer events occur,
/// which invoke the stored Lua functions.
pub struct InputCallbacks {
    pub on_click: Vec<LuaFunction>,
    pub on_scroll: Vec<LuaFunction>,
    pub on_hover: Vec<LuaFunction>,
    pub cursor_type: CursorType,
}

impl Default for InputCallbacks {
    fn default() -> Self {
        Self {
            on_click: Vec::new(),
            on_scroll: Vec::new(),
            on_hover: Vec::new(),
            cursor_type: CursorType::Default,
        }
    }
}

/// Shared registry of input callbacks, keyed by aura_id.
///
/// This is `Arc<Mutex<>>` so it can be shared between the Lua module
/// (which registers callbacks) and the renderer (which triggers them).
#[derive(Clone)]
pub struct InputRegistry {
    inner: Arc<Mutex<HashMap<String, InputCallbacks>>>,
}

impl InputRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Lock the inner map, recovering from poisoning.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, InputCallbacks>> {
        match self.inner.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("InputRegistry mutex poisoned, recovering");
                poisoned.into_inner()
            }
        }
    }

    /// Ensure callbacks exist for the given aura_id.
    fn ensure_aura(&self, aura_id: &str) {
        let mut map = self.lock();
        map.entry(aura_id.to_string()).or_default();
    }

    /// Add an `on_click` callback for the given aura.
    pub fn add_on_click(&self, aura_id: &str, cb: LuaFunction) {
        self.ensure_aura(aura_id);
        let mut map = self.lock();
        if let Some(entry) = map.get_mut(aura_id) {
            entry.on_click.push(cb);
        }
    }

    /// Add an `on_scroll` callback for the given aura.
    pub fn add_on_scroll(&self, aura_id: &str, cb: LuaFunction) {
        self.ensure_aura(aura_id);
        let mut map = self.lock();
        if let Some(entry) = map.get_mut(aura_id) {
            entry.on_scroll.push(cb);
        }
    }

    /// Add an `on_hover` callback for the given aura.
    pub fn add_on_hover(&self, aura_id: &str, cb: LuaFunction) {
        self.ensure_aura(aura_id);
        let mut map = self.lock();
        if let Some(entry) = map.get_mut(aura_id) {
            entry.on_hover.push(cb);
        }
    }

    /// Set the cursor type for the given aura.
    pub fn set_cursor(&self, aura_id: &str, cursor: &str) {
        self.ensure_aura(aura_id);
        let mut map = self.lock();
        if let Some(entry) = map.get_mut(aura_id) {
            entry.cursor_type = CursorType::from_str(cursor);
        }
    }

    /// Get the cursor type for the given aura.
    #[allow(dead_code)]
    pub fn get_cursor(&self, aura_id: &str) -> CursorType {
        let map = self.lock();
        map.get(aura_id)
            .map(|e| e.cursor_type.clone())
            .unwrap_or_default()
    }

    /// Trigger all `on_click` callbacks for the given aura.
    ///
    /// Arguments passed to Lua: `(button, x, y, component_name)`
    pub fn trigger_on_click(
        &self,
        aura_id: &str,
        button: u32,
        x: f32,
        y: f32,
        component_name: &str,
    ) {
        let callbacks: Vec<LuaFunction> = {
            let map = self.lock();
            map.get(aura_id)
                .map(|e| e.on_click.clone())
                .unwrap_or_default()
        };

        let component = component_name.to_string();
        for cb in callbacks {
            if let Err(e) = cb.call::<()>((button, x, y, component.clone())) {
                warn!("waio.input.on_click callback error: {}", e);
            }
        }
    }

    /// Trigger all `on_scroll` callbacks for the given aura.
    ///
    /// Arguments passed to Lua: `(delta_x, delta_y, x, y)`
    pub fn trigger_on_scroll(&self, aura_id: &str, delta_x: f32, delta_y: f32, x: f32, y: f32) {
        let callbacks: Vec<LuaFunction> = {
            let map = self.lock();
            map.get(aura_id)
                .map(|e| e.on_scroll.clone())
                .unwrap_or_default()
        };

        for cb in callbacks {
            if let Err(e) = cb.call::<()>((delta_x, delta_y, x, y)) {
                warn!("waio.input.on_scroll callback error: {}", e);
            }
        }
    }

    /// Trigger all `on_hover` callbacks for the given aura.
    ///
    /// Arguments passed to Lua: `(x, y, entered)` where `entered` is a boolean.
    pub fn trigger_on_hover(&self, aura_id: &str, x: f32, y: f32, entered: bool) {
        let callbacks: Vec<LuaFunction> = {
            let map = self.lock();
            map.get(aura_id)
                .map(|e| e.on_hover.clone())
                .unwrap_or_default()
        };

        for cb in callbacks {
            if let Err(e) = cb.call::<()>((x, y, entered)) {
                warn!("waio.input.on_hover callback error: {}", e);
            }
        }
    }

    /// Remove all callbacks for the given aura (cleanup on unload).
    pub fn clear_aura(&self, aura_id: &str) {
        let mut map = self.lock();
        map.remove(aura_id);
    }

    /// Remove all callbacks (cleanup on shutdown).
    pub fn clear_all(&self) {
        let mut map = self.lock();
        map.clear();
    }
}

/// Creates the Lua `waio.input` module.
///
/// ```lua
/// waio.input.on_click(callback)    -- callback(button, x, y, component_name)
/// waio.input.on_scroll(callback)   -- callback(delta_x, delta_y, x, y)
/// waio.input.on_hover(callback)    -- callback(x, y, entered)
/// waio.input.set_cursor(cursor)    -- cursor: "default", "pointer", "text", "grab", "crosshair", "wait"
/// waio.input.get_layout()          -- returns current keyboard layout, e.g. "en", "ru"
/// ```
pub fn create_module(lua: &Lua, registry: InputRegistry, aura_id: String) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    // on_click(callback)
    let reg_click = registry.clone();
    let aura_click = aura_id.clone();
    m.set(
        "on_click",
        lua.create_function(move |_, cb: LuaFunction| {
            reg_click.add_on_click(&aura_click, cb);
            debug!("Registered on_click callback for aura: {}", aura_click);
            Ok(())
        })?,
    )?;

    // on_scroll(callback)
    let reg_scroll = registry.clone();
    let aura_scroll = aura_id.clone();
    m.set(
        "on_scroll",
        lua.create_function(move |_, cb: LuaFunction| {
            reg_scroll.add_on_scroll(&aura_scroll, cb);
            debug!("Registered on_scroll callback for aura: {}", aura_scroll);
            Ok(())
        })?,
    )?;

    // on_hover(callback)
    let reg_hover = registry.clone();
    let aura_hover = aura_id.clone();
    m.set(
        "on_hover",
        lua.create_function(move |_, cb: LuaFunction| {
            reg_hover.add_on_hover(&aura_hover, cb);
            debug!("Registered on_hover callback for aura: {}", aura_hover);
            Ok(())
        })?,
    )?;

    // set_cursor(cursor)
    let reg_cursor = registry;
    let aura_cursor = aura_id;
    m.set(
        "set_cursor",
        lua.create_function(move |_, cursor: String| {
            reg_cursor.set_cursor(&aura_cursor, &cursor);
            debug!("Set cursor to '{}' for aura: {}", cursor, aura_cursor);
            Ok(())
        })?,
    )?;

    // get_layout()
    m.set(
        "get_layout",
        lua.create_function(|_, _: ()| {
            let layout = get_keyboard_layout();
            debug!("Keyboard layout: {}", layout);
            Ok(layout)
        })?,
    )?;

    Ok(m)
}
