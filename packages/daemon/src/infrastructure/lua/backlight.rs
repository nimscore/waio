//! `waio.backlight` module — screen brightness control via sysfs.
//!
//! Only exposed when the aura declares `permissions: [system]` in its `.wa` file.
//! Uses `/sys/class/backlight/` to find the first backlight device.
//!
//! Usage:
//! ```lua
//! local current = waio.backlight.get()
//! local max = waio.backlight.get_max()
//! waio.backlight.set(500)
//! waio.backlight.change(10)   -- increase by 10% of max
//! waio.backlight.change(-10)  -- decrease by 10% of max
//! ```

use mlua::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

use super::rate_limiter::RateLimiter;

const BACKLIGHT_BASE: &str = "/sys/class/backlight";

/// Find the first backlight device directory under `/sys/class/backlight/`.
fn find_backlight_device() -> Option<PathBuf> {
    let base = Path::new(BACKLIGHT_BASE);
    if !base.is_dir() {
        return None;
    }
    for entry in fs::read_dir(base).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_dir() && path.join("brightness").exists() && path.join("max_brightness").exists()
        {
            return Some(path);
        }
    }
    None
}

/// Read the current brightness value.
fn read_brightness(device_path: &Path) -> LuaResult<u64> {
    let content = fs::read_to_string(device_path.join("brightness"))
        .map_err(|e| mlua::Error::external(format!("Failed to read brightness: {}", e)))?;
    content
        .trim()
        .parse::<u64>()
        .map_err(|e| mlua::Error::external(format!("Invalid brightness value: {}", e)))
}

/// Read the max brightness value.
fn read_max_brightness(device_path: &Path) -> LuaResult<u64> {
    let content = fs::read_to_string(device_path.join("max_brightness"))
        .map_err(|e| mlua::Error::external(format!("Failed to read max_brightness: {}", e)))?;
    content
        .trim()
        .parse::<u64>()
        .map_err(|e| mlua::Error::external(format!("Invalid max_brightness value: {}", e)))
}

/// Write a brightness value.
fn write_brightness(device_path: &Path, value: u64) -> LuaResult<()> {
    fs::write(device_path.join("brightness"), format!("{}", value))
        .map_err(|e| mlua::Error::external(format!("Failed to write brightness: {}", e)))
}

/// Create the `waio.backlight` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    // waio.backlight.get() -> number | nil
    let rl_get = rate_limiter.clone();
    m.set(
        "get",
        lua.create_function(move |_, ()| -> LuaResult<Option<u64>> {
            if let Some(ref rl) = rl_get {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            let device = find_backlight_device();
            match device {
                Some(path) => {
                    let val = read_brightness(&path)?;
                    Ok(Some(val))
                }
                None => Ok(None),
            }
        })?,
    )?;

    // waio.backlight.get_max() -> number | nil
    let rl_get_max = rate_limiter.clone();
    m.set(
        "get_max",
        lua.create_function(move |_, ()| -> LuaResult<Option<u64>> {
            if let Some(ref rl) = rl_get_max {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            let device = find_backlight_device();
            match device {
                Some(path) => {
                    let val = read_max_brightness(&path)?;
                    Ok(Some(val))
                }
                None => Ok(None),
            }
        })?,
    )?;

    // waio.backlight.set(value)
    let rl_set = rate_limiter.clone();
    m.set(
        "set",
        lua.create_function(move |_, value: u64| -> LuaResult<()> {
            if let Some(ref rl) = rl_set {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            let device = find_backlight_device()
                .ok_or_else(|| mlua::Error::external("No backlight device found"))?;
            let max = read_max_brightness(&device)?;
            let clamped = value.min(max);
            write_brightness(&device, clamped)?;
            Ok(())
        })?,
    )?;

    // waio.backlight.change(delta) — delta is -100 to +100 percent
    let rl_change = rate_limiter;
    m.set(
        "change",
        lua.create_function(move |_, delta: f64| -> LuaResult<()> {
            if let Some(ref rl) = rl_change {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            let device = find_backlight_device()
                .ok_or_else(|| mlua::Error::external("No backlight device found"))?;
            let current = read_brightness(&device)? as f64;
            let max = read_max_brightness(&device)? as f64;

            let new_value = current + (max * delta / 100.0);
            let clamped = new_value.clamp(0.0, max) as u64;

            write_brightness(&device, clamped)?;
            Ok(())
        })?,
    )?;

    Ok(m)
}
