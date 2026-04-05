//! `waio.fs` module — safe read-only file access for Lua widgets.
//!
//! Only exposed when the aura declares `permissions: [fs_read]` in its `.wa` file.
//! Path traversal (`../../etc/passwd`) is blocked via `canonicalize()` + `starts_with()`.

use mlua::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Allowed root directories for read-only file access.
#[derive(Clone)]
pub struct FsAccess {
    allowed_roots: Vec<PathBuf>,
}

impl FsAccess {
    pub fn new(allowed_roots: Vec<PathBuf>) -> Self {
        Self { allowed_roots }
    }

    /// Check if the canonical form of `path` starts with any allowed root.
    fn is_path_allowed(&self, path: &Path) -> bool {
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => return false,
        };
        self.allowed_roots.iter().any(|root| {
            if let Ok(canonical_root) = root.canonicalize() {
                canonical.starts_with(&canonical_root)
            } else {
                false
            }
        })
    }

    /// Read file contents as string, with path validation.
    pub fn read_file(&self, path: &str) -> std::io::Result<String> {
        let path = Path::new(path);
        if !self.is_path_allowed(path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Path not allowed: {}", path.display()),
            ));
        }
        fs::read_to_string(path)
    }

    /// List directory entries (names only), with path validation.
    pub fn list_dir(&self, path: &str) -> std::io::Result<Vec<String>> {
        let path = Path::new(path);
        if !self.is_path_allowed(path) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Path not allowed: {}", path.display()),
            ));
        }
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                entries.push(name.to_string());
            }
        }
        entries.sort();
        Ok(entries)
    }
}

/// Create the `waio.fs` Lua module with `read_file(path)` and `list_dir(path)`.
pub fn create_module(lua: &Lua, fs_access: FsAccess) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let fs_for_read = fs_access.clone();
    m.set(
        "read_file",
        lua.create_function(move |_, path: String| -> LuaResult<String> {
            fs_for_read.read_file(&path)
                .map_err(|e| mlua::Error::external(e.to_string()))
        })?,
    )?;

    let fs_for_list = fs_access;
    m.set(
        "list_dir",
        lua.create_function(move |_, path: String| -> LuaResult<Vec<String>> {
            fs_for_list.list_dir(&path)
                .map_err(|e| mlua::Error::external(e.to_string()))
        })?,
    )?;

    Ok(m)
}
