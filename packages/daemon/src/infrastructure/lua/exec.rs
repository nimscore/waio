//! `waio.exec` module — controlled system command execution for Lua widgets.
//!
//! Only exposed when the aura declares `exec` permission in its `.wa` file.
//! The permission can be:
//! - `exec: true` — allow all commands (dangerous, development only)
//! - `exec: [kitty, swaymsg, ...]` — whitelist of allowed commands
//!
//! Security controls:
//! - Command whitelist (empty = allow all)
//! - No stdin (`Stdio::null()`)
//! - 30-second hard timeout via `tokio::time::timeout`
//! - `kill_on_drop(true)` to prevent zombie processes

use mlua::prelude::*;
use std::process::Stdio;
use std::time::Duration;

use super::rate_limiter::RateLimiter;

const EXEC_TIMEOUT: Duration = Duration::from_secs(30);

/// Command execution access configuration for a widget.
#[derive(Clone)]
pub struct ExecAccess {
    /// Allowed commands (basename only). If empty, all commands are allowed.
    allowed_commands: Vec<String>,
}

impl ExecAccess {
    pub fn new(allowed_commands: Vec<String>) -> Self {
        Self { allowed_commands }
    }

    /// Check if a command is allowed.
    /// When `allowed_commands` is empty, all commands are allowed (dev mode).
    pub fn is_command_allowed(&self, cmd: &str) -> bool {
        if self.allowed_commands.is_empty() {
            return true;
        }

        // Check both the exact command and its basename.
        let basename = std::path::Path::new(cmd)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| cmd.to_string());

        self.allowed_commands.contains(&cmd.to_string())
            || self.allowed_commands.contains(&basename)
    }

    /// Run a command with security controls and timeout.
    /// Returns `(stdout, stderr, exit_code)`.
    async fn run_command(&self, cmd: &str, args: &[String]) -> LuaResult<(String, String, i32)> {
        if !self.is_command_allowed(cmd) {
            return Err(mlua::Error::external(format!(
                "Command not allowed: '{}'. Add it to the exec whitelist.",
                cmd
            )));
        }

        let mut command = tokio::process::Command::new(cmd);
        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let output_result = tokio::time::timeout(EXEC_TIMEOUT, command.output()).await;

        let output = match output_result {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                return Err(mlua::Error::external(format!(
                    "Failed to execute command: {}",
                    e
                )));
            }
            Err(_) => {
                return Err(mlua::Error::external(format!(
                    "Command timed out after {} seconds",
                    EXEC_TIMEOUT.as_secs()
                )));
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, status))
    }
}

/// Create the `waio.exec` Lua module with `exec(cmd, args?)`.
pub fn create_module(
    lua: &Lua,
    exec_access: ExecAccess,
    rate_limiter: Option<RateLimiter>,
) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let exec = exec_access.clone();
    let rl = rate_limiter;
    m.set(
        "exec",
        lua.create_function(
            move |lua, (cmd, args): (String, Option<Vec<String>>)| -> LuaResult<LuaTable> {
                if let Some(ref rl) = rl {
                    rl.check_and_record("exec")
                        .map_err(|e| mlua::Error::external(e.to_string()))?;
                }

                let args = args.unwrap_or_default();

                // Run async command synchronously using spin_on.
                let result = spin_on::spin_on(exec.run_command(&cmd, &args));

                let (stdout, stderr, status) = result?;

                let result = lua.create_table()?;
                result.set("stdout", stdout)?;
                result.set("stderr", stderr)?;
                result.set("status", status)?;

                Ok(result)
            },
        )?,
    )?;

    Ok(m)
}
