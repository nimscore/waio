//! `waio.clipboard` module — clipboard operations via wl-clipboard.
//!
//! Provides:
//! - `waio.clipboard.get()` — read text from clipboard (string or nil on error)
//! - `waio.clipboard.set(text)` — copy text to clipboard
//!
//! Uses `wl-paste` and `wl-copy` CLI tools from the wl-clipboard package.
//! These are standard Wayland clipboard tools and are SAFE to expose without
//! additional permissions.

use mlua::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const CLIPBOARD_TIMEOUT: Duration = Duration::from_secs(5);

/// Read text from the clipboard using `wl-paste`.
fn read_clipboard() -> std::io::Result<String> {
    let output = Command::new("wl-paste")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        Ok(text.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::other(format!(
            "wl-paste failed: {}",
            stderr.trim()
        )))
    }
}

/// Write text to the clipboard using `wl-copy`.
fn write_clipboard(text: &str) -> std::io::Result<()> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(std::io::Error::other("wl-copy failed"));
    }

    Ok(())
}

/// Run a closure in a separate thread with a timeout.
fn with_timeout<T, F>(f: F, timeout: Duration) -> Result<T, String>
where
    F: FnOnce() -> Result<T, std::io::Error> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel::<Result<T, std::io::Error>>();

    thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });

    match rx.recv_timeout(timeout) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(e)) => Err(e.to_string()),
        Err(_) => Err(format!("Clipboard operation timed out after {:?}", timeout)),
    }
}

/// Create the `waio.clipboard` Lua module.
pub fn create_module(lua: &Lua) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    m.set(
        "get",
        lua.create_function(|_lua, (): ()| -> LuaResult<Option<String>> {
            match with_timeout(read_clipboard, CLIPBOARD_TIMEOUT) {
                Ok(text) => {
                    if text.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(text))
                    }
                }
                Err(e) => {
                    tracing::warn!("Clipboard read failed: {}", e);
                    Ok(None)
                }
            }
        })?,
    )?;

    m.set(
        "set",
        lua.create_function(|_lua, text: String| -> LuaResult<()> {
            with_timeout(move || write_clipboard(&text), CLIPBOARD_TIMEOUT).map_err(|e| {
                tracing::warn!("Clipboard write failed: {}", e);
                mlua::Error::external(format!("Failed to copy to clipboard: {}", e))
            })
        })?,
    )?;

    Ok(m)
}
