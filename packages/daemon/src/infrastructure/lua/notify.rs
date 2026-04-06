//! `waio.notify` module — desktop notifications for Lua widgets.
//!
//! Only exposed when the aura declares `permissions: [system]` in its `.wa` file.
//! Uses `notify-rust` with the freedesktop notifications specification.
//!
//! Usage:
//! ```lua
//! waio.notify({
//!     title = "Notification title",
//!     body = "Notification body",
//!     icon = "dialog-information",  -- optional
//!     urgency = "normal",           -- optional: "low", "normal", "critical"
//!     timeout = 5000                -- optional: milliseconds, default 5000
//! })
//! ```

use mlua::prelude::*;
use notify_rust::{Notification, Timeout, Urgency};

use super::rate_limiter::RateLimiter;

const DEFAULT_ICON: &str = "dialog-information";
const DEFAULT_TIMEOUT_MS: u32 = 5000;

/// Create the `waio.notify` Lua module.
pub fn create_module(lua: &Lua, rate_limiter: Option<RateLimiter>) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let rl = rate_limiter;
    m.set(
        "notify",
        lua.create_function(move |_, opts: LuaTable| -> LuaResult<()> {
            if let Some(ref rl) = rl {
                rl.check_and_record("notify")
                    .map_err(|e| mlua::Error::external(e.to_string()))?;
            }

            let title: Option<String> = opts.get("title").ok();
            let body: Option<String> = opts.get("body").ok();
            let icon: String = opts
                .get("icon")
                .unwrap_or_else(|_| DEFAULT_ICON.to_string());
            let urgency_str: String = opts.get("urgency").unwrap_or_else(|_| "normal".to_string());
            let timeout_ms: u32 = opts.get("timeout").unwrap_or(DEFAULT_TIMEOUT_MS);

            let urgency = match urgency_str.as_str() {
                "low" => Urgency::Low,
                "critical" => Urgency::Critical,
                _ => Urgency::Normal,
            };

            let summary = title.unwrap_or_default();
            let body = body.unwrap_or_default();

            Notification::new()
                .summary(&summary)
                .body(&body)
                .icon(&icon)
                .urgency(urgency)
                .timeout(Timeout::Milliseconds(timeout_ms))
                .show()
                .map_err(|e| mlua::Error::external(e.to_string()))?;

            Ok(())
        })?,
    )?;

    Ok(m)
}
