//! `waio.bluetooth` module — Bluetooth control via `bluetoothctl` (BlueZ).
//!
//! Provides functions to scan, connect, disconnect, enable/disable Bluetooth,
//! and check adapter status.
//!
//! Requires the `system` permission.

use mlua::prelude::*;
use std::process::Command;

use super::rate_limiter::RateLimiter;

/// Run `bluetoothctl` with the given arguments and return stdout on success.
fn run_bluetoothctl(args: &[&str]) -> Result<String, String> {
    let output = Command::new("bluetoothctl")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute bluetoothctl: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("bluetoothctl failed: {}", stderr.trim()));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in bluetoothctl output: {}", e))
}

/// Pipe commands to bluetoothctl via stdin and return stdout.
fn pipe_bluetoothctl(input: &str) -> Result<String, String> {
    let mut child = std::process::Command::new("bluetoothctl")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute bluetoothctl: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin
            .write_all(input.as_bytes())
            .map_err(|e| format!("Failed to write to bluetoothctl stdin: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to read bluetoothctl output: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // bluetoothctl often exits non-zero even on success, so only error on empty stdout
        if output.stdout.is_empty() {
            return Err(format!("bluetoothctl failed: {}", stderr.trim()));
        }
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 in bluetoothctl output: {}", e))
}

/// Scan for Bluetooth devices.
/// Returns a list of `{address, name, paired, connected, battery?}` tables.
fn scan(_lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    // Get list of devices.
    let devices_output =
        run_bluetoothctl(&["--timeout", "10", "devices"]).map_err(mlua::Error::external)?;

    let result = _lua.create_table()?;
    let mut index: i32 = 1;

    for line in devices_output.lines() {
        // Parse "Device AA:BB:CC:DD:EE:FF My Device"
        if !line.starts_with("Device ") {
            continue;
        }

        let rest = &line["Device ".len()..];
        let parts: Vec<&str> = rest.splitn(2, ' ').collect();
        if parts.len() < 2 {
            continue;
        }

        let address = parts[0];
        let name = parts[1];

        // Get detailed info for this device.
        let info_output = run_bluetoothctl(&["info", address]);

        let paired = if let Ok(ref info) = info_output {
            info.lines().any(|l| l.trim().starts_with("Paired: yes"))
        } else {
            false
        };

        let connected = if let Ok(ref info) = info_output {
            info.lines().any(|l| l.trim().starts_with("Connected: yes"))
        } else {
            false
        };

        let battery = if let Ok(ref info) = info_output {
            info.lines().find_map(|l| {
                let trimmed = l.trim();
                if let Some(value) = trimmed.strip_prefix("Battery: ") {
                    // Parse "85%" → 85
                    value.trim_end_matches('%').parse::<i32>().ok()
                } else {
                    None
                }
            })
        } else {
            None
        };

        let entry = _lua.create_table()?;
        entry.set("address", address.to_string())?;
        entry.set("name", name.to_string())?;
        entry.set("paired", paired)?;
        entry.set("connected", connected)?;
        if let Some(bat) = battery {
            entry.set("battery", bat)?;
        }

        result.set(index, entry)?;
        index += 1;
    }

    Ok(result)
}

/// Connect to a Bluetooth device.
/// Args: (address: string)
/// Returns: {ok: boolean, error: string?}
fn connect(lua: &Lua, address: String) -> LuaResult<LuaTable> {
    let input = format!("connect {}\nquit\n", address);
    let output = pipe_bluetoothctl(&input).map_err(mlua::Error::external)?;

    let ret = lua.create_table()?;
    if output.contains("Connection successful") {
        ret.set("ok", true)?;
    } else {
        ret.set("ok", false)?;
        // Extract error-like lines from output
        let error_msg = output
            .lines()
            .find(|l| {
                l.contains("Failed")
                    || l.contains("failed")
                    || l.contains("Error")
                    || l.contains("Not available")
            })
            .map(|l| l.trim().to_string())
            .unwrap_or_else(|| "Connection failed".to_string());
        ret.set("error", error_msg)?;
    }

    Ok(ret)
}

/// Disconnect from a Bluetooth device.
/// Args: (address: string)
/// Returns: {ok: boolean, error: string?}
fn disconnect(lua: &Lua, address: String) -> LuaResult<LuaTable> {
    let input = format!("disconnect {}\nquit\n", address);
    let output = pipe_bluetoothctl(&input).map_err(mlua::Error::external)?;

    let ret = lua.create_table()?;
    if output.contains("Successful disconnected") || output.contains("Disconnected") {
        ret.set("ok", true)?;
    } else {
        ret.set("ok", false)?;
        let error_msg = output
            .lines()
            .find(|l| l.contains("Failed") || l.contains("failed") || l.contains("Error"))
            .map(|l| l.trim().to_string())
            .unwrap_or_else(|| "Not connected".to_string());
        ret.set("error", error_msg)?;
    }

    Ok(ret)
}

/// Enable or disable Bluetooth.
/// Args: (enabled: boolean)
/// Returns: {ok: boolean, error: string?}
fn set_enabled(lua: &Lua, enabled: bool) -> LuaResult<LuaTable> {
    let power_cmd = if enabled { "power on" } else { "power off" };
    let input = format!("{}\nquit\n", power_cmd);
    let output = pipe_bluetoothctl(&input);

    let ret = lua.create_table()?;
    match output {
        Ok(out) => {
            // bluetoothctl may still return non-zero, check for expected output
            let success_indicator = if enabled {
                "Changing power on succeeded"
            } else {
                "Changing power off succeeded"
            };

            if out.contains(success_indicator) {
                ret.set("ok", true)?;
            } else {
                ret.set("ok", false)?;
                let error_msg = out
                    .lines()
                    .find(|l| l.contains("Failed") || l.contains("failed"))
                    .map(|l| l.trim().to_string())
                    .unwrap_or_else(|| {
                        format!(
                            "Failed to {} Bluetooth",
                            if enabled { "enable" } else { "disable" }
                        )
                    });
                ret.set("error", error_msg)?;
            }
        }
        Err(e) => {
            ret.set("ok", false)?;
            ret.set("error", e)?;
        }
    }

    Ok(ret)
}

/// Get Bluetooth adapter status.
/// Returns: {powered: boolean, discovering: boolean}
fn status(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let input = "show\nquit\n".to_string();
    let output = pipe_bluetoothctl(&input).map_err(mlua::Error::external)?;

    let powered = output
        .lines()
        .find(|l| l.trim().starts_with("Powered: "))
        .map(|l| l.trim().ends_with("yes"))
        .unwrap_or(false);

    let discovering = output
        .lines()
        .find(|l| l.trim().starts_with("Discovering: "))
        .map(|l| l.trim().ends_with("yes"))
        .unwrap_or(false);

    let ret = lua.create_table()?;
    ret.set("powered", powered)?;
    ret.set("discovering", discovering)?;

    Ok(ret)
}

/// Create the `waio.bluetooth` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let rl_scan = rate_limiter.clone();
    m.set(
        "scan",
        lua.create_function(move |lua, args: ()| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_scan {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            scan(lua, args)
        })?,
    )?;

    let rl_connect = rate_limiter.clone();
    m.set(
        "connect",
        lua.create_function(move |lua, address: String| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_connect {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            connect(lua, address)
        })?,
    )?;

    let rl_disconnect = rate_limiter.clone();
    m.set(
        "disconnect",
        lua.create_function(move |lua, address: String| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_disconnect {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            disconnect(lua, address)
        })?,
    )?;

    let rl_enabled = rate_limiter.clone();
    m.set(
        "enabled",
        lua.create_function(move |lua, enabled: bool| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_enabled {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            set_enabled(lua, enabled)
        })?,
    )?;

    let rl_status = rate_limiter;
    m.set(
        "status",
        lua.create_function(move |lua, args: ()| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_status {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            status(lua, args)
        })?,
    )?;

    Ok(m)
}
