//! `waio.power` module — power management via systemctl/loginctl.
//!
//! Only exposed when the aura declares `permissions: [system]` in its `.wa` file.
//!
//! Usage:
//! ```lua
//! waio.power.shutdown()     -- Power off
//! waio.power.reboot()       -- Reboot
//! waio.power.suspend()      -- Suspend to RAM
//! waio.power.hibernate()    -- Suspend to disk
//! waio.power.lock_screen()  -- Lock screen (swaylock or loginctl)
//! ```

use mlua::prelude::*;
use std::process::Command;

use super::rate_limiter::RateLimiter;

/// Run a command, returning a Lua error on failure.
fn run_cmd(program: &str, args: &[&str], error_label: &str) -> LuaResult<()> {
    Command::new(program)
        .args(args)
        .status()
        .map_err(|e| mlua::Error::external(format!("{}: {}", error_label, e)))?;
    Ok(())
}

/// Create the `waio.power` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    // waio.power.shutdown()
    let rl_shutdown = rate_limiter.clone();
    m.set(
        "shutdown",
        lua.create_function(move |_, ()| -> LuaResult<()> {
            if let Some(ref rl) = rl_shutdown {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            run_cmd("systemctl", &["poweroff"], "Failed to shutdown")
        })?,
    )?;

    // waio.power.reboot()
    let rl_reboot = rate_limiter.clone();
    m.set(
        "reboot",
        lua.create_function(move |_, ()| -> LuaResult<()> {
            if let Some(ref rl) = rl_reboot {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            run_cmd("systemctl", &["reboot"], "Failed to reboot")
        })?,
    )?;

    // waio.power.suspend()
    let rl_suspend = rate_limiter.clone();
    m.set(
        "suspend",
        lua.create_function(move |_, ()| -> LuaResult<()> {
            if let Some(ref rl) = rl_suspend {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            run_cmd("systemctl", &["suspend"], "Failed to suspend")
        })?,
    )?;

    // waio.power.hibernate()
    let rl_hibernate = rate_limiter.clone();
    m.set(
        "hibernate",
        lua.create_function(move |_, ()| -> LuaResult<()> {
            if let Some(ref rl) = rl_hibernate {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }
            run_cmd("systemctl", &["hibernate"], "Failed to hibernate")
        })?,
    )?;

    // waio.power.lock_screen() — try swaylock, fallback to loginctl
    let rl_lock = rate_limiter;
    m.set(
        "lock_screen",
        lua.create_function(move |_, ()| -> LuaResult<()> {
            if let Some(ref rl) = rl_lock {
                rl.check_and_record("system")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }

            // Try swaylock first.
            let swaylock = Command::new("swaylock").status().map(|s| s.success());

            if let Ok(true) = swaylock {
                return Ok(());
            }

            // Fallback: loginctl lock-session.
            run_cmd("loginctl", &["lock-session"], "Failed to lock screen")
        })?,
    )?;

    Ok(m)
}
