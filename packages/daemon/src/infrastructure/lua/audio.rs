//! `waio.audio` module — audio volume control via `pactl` (PulseAudio/PipeWire).
//!
//! Provides functions to get/set volume and mute state for the default sink,
//! as well as listing all available sinks.
//!
//! Requires the `system` permission.

use mlua::prelude::*;
use std::process::Command;

use super::rate_limiter::RateLimiter;

/// Sink properties returned by `list_sinks`.
pub struct SinkInfo {
    pub name: String,
    pub description: String,
    pub volume: f64,
    pub muted: bool,
    pub is_default: bool,
}

/// Run a `pactl` command and return its stdout.
fn run_pactl(args: &[&str]) -> Result<String, String> {
    let output = Command::new("pactl")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute pactl: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("pactl failed: {}", stderr.trim()));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in pactl output: {}", e))
}

/// Get the default sink name.
fn get_default_sink() -> Result<String, String> {
    let output = run_pactl(&["get-default-sink"])?;
    // Output format: "Name: <sink_name>"
    for line in output.lines() {
        if let Some(stripped) = line.strip_prefix("Name:") {
            return Ok(stripped.trim().to_string());
        }
    }
    Err("Could not determine default sink".to_string())
}

/// Get volume of the default sink. Returns 0-100+.
fn get_volume() -> Result<f64, String> {
    let sink = get_default_sink()?;
    let output = run_pactl(&["get-sink-volume", &sink])?;
    // Output format: "Volume: front-left: 45000 /  69% / -9.69 dB, ..."
    // We parse the first channel's percentage
    if let Some(vol_pos) = output.find('%') {
        // Find the number before %
        let before = &output[..vol_pos];
        let start = before
            .rfind(|c: char| !c.is_ascii_digit() && c != '.')
            .unwrap_or(0);
        let num_str = before[start..].trim();
        if let Ok(val) = num_str.parse::<f64>() {
            return Ok(val);
        }
    }
    Err("Could not parse volume from pactl output".to_string())
}

/// Set volume of the default sink (0-100+).
fn set_volume(percent: f64) -> Result<(), String> {
    let sink = get_default_sink()?;
    run_pactl(&["set-sink-volume", &sink, &format!("{}%", percent as i64)])?;
    Ok(())
}

/// Get mute state of the default sink.
fn get_mute() -> Result<bool, String> {
    let sink = get_default_sink()?;
    let output = run_pactl(&["get-sink-mute", &sink])?;
    // Output: "Mute: yes" or "Mute: no"
    for line in output.lines() {
        if let Some(stripped) = line.strip_prefix("Mute:") {
            let value = stripped.trim();
            return Ok(value == "yes");
        }
    }
    Err("Could not parse mute state from pactl output".to_string())
}

/// Set mute state of the default sink.
fn set_mute(muted: bool) -> Result<(), String> {
    let sink = get_default_sink()?;
    let state = if muted { "1" } else { "0" };
    run_pactl(&["set-sink-mute", &sink, state])?;
    Ok(())
}

/// List all sinks with their properties.
fn list_sinks() -> Result<Vec<SinkInfo>, String> {
    let output = run_pactl(&["list-sinks"])?;

    let mut sinks: Vec<SinkInfo> = Vec::new();
    let mut current_name = String::new();
    let mut current_desc = String::new();
    let mut current_vol = 0.0;
    let mut current_muted = false;
    let mut current_default = false;
    let mut in_sink = false;

    // Get default sink name for comparison
    let default_sink_name = get_default_sink().ok();

    for line in output.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Sink #") {
            // Save previous sink if any
            if in_sink && !current_name.is_empty() {
                sinks.push(SinkInfo {
                    name: current_name.clone(),
                    description: current_desc.clone(),
                    volume: current_vol,
                    muted: current_muted,
                    is_default: current_default,
                });
            }
            in_sink = true;
            current_name.clear();
            current_desc.clear();
            current_vol = 0.0;
            current_muted = false;
            current_default = false;
        } else if in_sink {
            if let Some(stripped) = trimmed.strip_prefix("Name:") {
                current_name = stripped.trim().to_string();
                if let Some(ref default) = default_sink_name {
                    current_default = &current_name == default;
                }
            } else if let Some(stripped) = trimmed.strip_prefix("Description:") {
                current_desc = stripped.trim().to_string();
            } else if trimmed.starts_with("Volume:") && current_vol == 0.0 {
                // Parse volume from first Volume: line
                if let Some(vol_pos) = trimmed.find('%') {
                    let before = &trimmed[..vol_pos];
                    let start = before
                        .rfind(|c: char| !c.is_ascii_digit() && c != '.')
                        .unwrap_or(0);
                    let num_str = before[start..].trim();
                    if let Ok(val) = num_str.parse::<f64>() {
                        current_vol = val;
                    }
                }
            } else if let Some(stripped) = trimmed.strip_prefix("Mute:") {
                current_muted = stripped.trim() == "yes";
            }
        }
    }

    // Don't forget the last sink
    if in_sink && !current_name.is_empty() {
        sinks.push(SinkInfo {
            name: current_name,
            description: current_desc,
            volume: current_vol,
            muted: current_muted,
            is_default: current_default,
        });
    }

    Ok(sinks)
}

/// Create the `waio.audio` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    // waio.audio.get_volume() -> number (0-100)
    let rl_get_vol = rate_limiter.clone();
    m.set(
        "get_volume",
        lua.create_function(move |_, _: ()| -> LuaResult<f64> {
            if let Some(ref rl) = rl_get_vol {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            get_volume().map_err(mlua::Error::external)
        })?,
    )?;

    // waio.audio.set_volume(percent) -> nil
    let rl_set_vol = rate_limiter.clone();
    m.set(
        "set_volume",
        lua.create_function(move |_, percent: f64| -> LuaResult<()> {
            if let Some(ref rl) = rl_set_vol {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            if percent < 0.0 {
                return Err(mlua::Error::external("Volume cannot be negative"));
            }
            set_volume(percent).map_err(mlua::Error::external)
        })?,
    )?;

    // waio.audio.get_mute() -> boolean
    let rl_get_mute = rate_limiter.clone();
    m.set(
        "get_mute",
        lua.create_function(move |_, _: ()| -> LuaResult<bool> {
            if let Some(ref rl) = rl_get_mute {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            get_mute().map_err(mlua::Error::external)
        })?,
    )?;

    // waio.audio.set_mute(bool) -> nil
    let rl_set_mute = rate_limiter.clone();
    m.set(
        "set_mute",
        lua.create_function(move |_, muted: bool| -> LuaResult<()> {
            if let Some(ref rl) = rl_set_mute {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            set_mute(muted).map_err(mlua::Error::external)
        })?,
    )?;

    // waio.audio.list_sinks() -> [{name, description, volume, muted, is_default}]
    let rl_list = rate_limiter;
    m.set(
        "list_sinks",
        lua.create_function(move |lua, _: ()| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_list {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            let sinks = list_sinks().map_err(mlua::Error::external)?;
            let result = lua.create_table()?;
            for (i, sink) in sinks.into_iter().enumerate() {
                let sink_info = lua.create_table()?;
                sink_info.set("name", sink.name)?;
                sink_info.set("description", sink.description)?;
                sink_info.set("volume", sink.volume)?;
                sink_info.set("muted", sink.muted)?;
                sink_info.set("is_default", sink.is_default)?;
                result.set((i + 1) as i64, sink_info)?;
            }
            Ok(result)
        })?,
    )?;

    Ok(m)
}
