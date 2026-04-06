//! `waio.wifi` module — WiFi control via `nmcli` (NetworkManager).
//!
//! Provides functions to scan, connect, disconnect, check current connection,
//! and enable/disable WiFi.
//!
//! Requires the `network` permission.

use mlua::prelude::*;
use std::process::Command;

use super::rate_limiter::RateLimiter;

/// Run `nmcli` and return stdout on success.
fn run_nmcli(args: &[&str]) -> Result<String, String> {
    let output = Command::new("nmcli")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute nmcli: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("nmcli failed: {}", stderr.trim()));
    }

    String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in nmcli output: {}", e))
}

/// Scan for available WiFi networks.
/// Returns a list of `{ssid, strength, security}` tables.
fn scan(_lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let output = run_nmcli(&["-t", "-f", "SSID,SIGNAL,SECURITY", "dev", "wifi", "list"])
        .map_err(mlua::Error::external)?;

    let result = _lua.create_table()?;
    let mut index: i32 = 1;

    for line in output.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse SSID:signal:security (colon-separated in terse mode)
        // But SSID itself might contain colons, so we split from the right.
        // Format: SSID:SIGNAL:SECURITY
        // We need at least 3 fields. Split into at most 3 parts.
        let parts: Vec<&str> = line.splitn(3, ':').collect();
        if parts.len() < 3 {
            continue;
        }

        let ssid = parts[0];
        let strength = parts[1];
        let security = parts[2];

        // Skip entries with empty SSID (hidden networks)
        if ssid.is_empty() {
            continue;
        }

        let entry = _lua.create_table()?;
        entry.set("ssid", ssid.to_string())?;
        entry.set("strength", strength.parse::<i32>().unwrap_or(0))?;
        entry.set("security", security.to_string())?;

        result.set(index, entry)?;
        index += 1;
    }

    Ok(result)
}

/// Connect to a WiFi network.
/// Args: (ssid: string, options?: {password: string?})
/// Returns: {ok: boolean, error: string?}
fn connect(lua: &Lua, args: (String, Option<LuaTable>)) -> LuaResult<LuaTable> {
    let (ssid, options) = args;
    let password = options.and_then(|t| t.get::<String>("password").ok());

    let mut cmd_args: Vec<&str> = vec!["dev", "wifi", "connect", &ssid];
    if let Some(ref pw) = password {
        cmd_args.push("password");
        cmd_args.push(pw);
    }

    let result = run_nmcli(&cmd_args);

    let ret = lua.create_table()?;
    match result {
        Ok(_) => {
            ret.set("ok", true)?;
        }
        Err(e) => {
            ret.set("ok", false)?;
            ret.set("error", e)?;
        }
    }

    Ok(ret)
}

/// Get the currently connected WiFi SSID.
/// Returns the SSID string or nil.
fn current(_lua: &Lua, _: ()) -> LuaResult<Option<String>> {
    let output =
        run_nmcli(&["-t", "-f", "ACTIVE,SSID", "dev", "wifi"]).map_err(mlua::Error::external)?;

    for line in output.lines() {
        if let Some(ssid) = line.strip_prefix("yes:") {
            if !ssid.is_empty() {
                return Ok(Some(ssid.to_string()));
            }
        }
    }

    Ok(None)
}

/// Disconnect from the current WiFi network.
/// Returns {ok: boolean, error: string?}
fn disconnect(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    // First find the active WiFi device
    let devices = run_nmcli(&["-t", "-f", "DEVICE,TYPE,STATE", "device"]);

    let ret = lua.create_table()?;
    match devices {
        Ok(output) => {
            // Find a WiFi device that is connected
            let mut wifi_device: Option<String> = None;
            for line in output.lines() {
                let parts: Vec<&str> = line.splitn(3, ':').collect();
                if parts.len() >= 3 && parts[1] == "wifi" && parts[2] == "connected" {
                    wifi_device = Some(parts[0].to_string());
                    break;
                }
            }

            if let Some(device) = wifi_device {
                let result = run_nmcli(&["dev", "disconnect", &device]);
                match result {
                    Ok(_) => {
                        ret.set("ok", true)?;
                    }
                    Err(e) => {
                        ret.set("ok", false)?;
                        ret.set("error", e)?;
                    }
                }
            } else {
                ret.set("ok", false)?;
                ret.set("error", "No connected WiFi device found")?;
            }
        }
        Err(e) => {
            ret.set("ok", false)?;
            ret.set("error", e)?;
        }
    }

    Ok(ret)
}

/// Enable or disable WiFi.
/// Args: (enabled: boolean)
/// Returns: {ok: boolean, error: string?}
fn set_enabled(lua: &Lua, enabled: bool) -> LuaResult<LuaTable> {
    let state = if enabled { "on" } else { "off" };
    let result = run_nmcli(&["radio", "wifi", state]);

    let ret = lua.create_table()?;
    match result {
        Ok(_) => {
            ret.set("ok", true)?;
        }
        Err(e) => {
            ret.set("ok", false)?;
            ret.set("error", e)?;
        }
    }

    Ok(ret)
}

/// Create the `waio.wifi` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let rl_scan = rate_limiter.clone();
    m.set(
        "scan",
        lua.create_function(move |lua, args: ()| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_scan {
                rl.check_and_record("network")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            scan(lua, args)
        })?,
    )?;

    let rl_connect = rate_limiter.clone();
    m.set(
        "connect",
        lua.create_function(
            move |lua, args: (String, Option<LuaTable>)| -> LuaResult<LuaTable> {
                if let Some(ref rl) = rl_connect {
                    rl.check_and_record("network")
                        .map_err(|e| mlua::Error::external(e.to_string()))?;
                }
                connect(lua, args)
            },
        )?,
    )?;

    let rl_current = rate_limiter.clone();
    m.set(
        "current",
        lua.create_function(move |lua, args: ()| -> LuaResult<Option<String>> {
            if let Some(ref rl) = rl_current {
                rl.check_and_record("network")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            current(lua, args)
        })?,
    )?;

    let rl_disconnect = rate_limiter.clone();
    m.set(
        "disconnect",
        lua.create_function(move |lua, args: ()| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_disconnect {
                rl.check_and_record("network")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            disconnect(lua, args)
        })?,
    )?;

    let rl_enabled = rate_limiter;
    m.set(
        "enabled",
        lua.create_function(move |lua, enabled: bool| -> LuaResult<LuaTable> {
            if let Some(ref rl) = rl_enabled {
                rl.check_and_record("network")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            set_enabled(lua, enabled)
        })?,
    )?;

    Ok(m)
}
