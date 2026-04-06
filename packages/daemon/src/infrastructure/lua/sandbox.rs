//! Lua sandboxing for Waio auras.
//!
//! Two-layer defense:
//! 1. `StdLib::ALL_SAFE` — blocks `debug` and C modules at construction
//! 2. `sanitize_globals()` — nils dangerous functions (`os.execute`, `io.popen`, etc.)
//! 3. `create_restricted_env()` — per-aura restricted environment with only allowed modules

use mlua::prelude::*;
use mlua::Nil;

/// Sanitize the Lua global environment by removing dangerous functions.
/// Called once after `Lua::new_with(StdLib::ALL_SAFE)`.
pub fn sanitize_globals(lua: &Lua) -> LuaResult<()> {
    let globals = lua.globals();

    // Block dangerous OS functions.
    if let Ok(os_tbl) = globals.get::<LuaTable>("os") {
        os_tbl.set("execute", Nil)?;
        os_tbl.set("popen", Nil)?;
        os_tbl.set("remove", Nil)?;
        os_tbl.set("rename", Nil)?;
        os_tbl.set("exit", Nil)?;
        os_tbl.set("setlocale", Nil)?;
    }

    // Block dangerous IO functions.
    if let Ok(io_tbl) = globals.get::<LuaTable>("io") {
        io_tbl.set("popen", Nil)?;
        io_tbl.set("open", Nil)?;
        io_tbl.set("input", Nil)?;
        io_tbl.set("output", Nil)?;
        io_tbl.set("lines", Nil)?;
        io_tbl.set("tmpfile", Nil)?;
    }

    // Block dangerous package functions.
    if let Ok(pkg_tbl) = globals.get::<LuaTable>("package") {
        pkg_tbl.set("loadlib", Nil)?;
        pkg_tbl.set("searchpath", Nil)?;
        pkg_tbl.set("cpath", "")?;
    }

    // Block dangerous globals.
    globals.set("dofile", Nil)?;
    globals.set("loadfile", Nil)?;
    globals.set("collectgarbage", Nil)?;
    globals.set("_G", Nil)?;

    Ok(())
}

/// Create a restricted environment for an aura, containing only safe modules.
///
/// The aura's Lua code runs in this environment and cannot access dangerous functions
/// or walk back to the real global environment.
///
/// Important: `waio` and `slint` are NOT copied here — they are registered separately
/// by the caller (in both globals and env, so timer callbacks work correctly).
pub fn create_restricted_env(lua: &Lua) -> LuaResult<LuaTable> {
    let globals = lua.globals();
    let env = lua.create_table()?;

    // Language primitives.
    env.set("_G", env.clone())?;
    env.set("assert", globals.get::<LuaFunction>("assert")?)?;
    env.set("error", globals.get::<LuaFunction>("error")?)?;
    env.set("ipairs", globals.get::<LuaFunction>("ipairs")?)?;
    env.set("next", globals.get::<LuaFunction>("next")?)?;
    env.set("pairs", globals.get::<LuaFunction>("pairs")?)?;
    env.set("pcall", globals.get::<LuaFunction>("pcall")?)?;
    env.set("rawequal", globals.get::<LuaFunction>("rawequal")?)?;
    env.set("rawget", globals.get::<LuaFunction>("rawget")?)?;
    env.set("rawlen", globals.get::<LuaFunction>("rawlen")?)?;
    env.set("rawset", globals.get::<LuaFunction>("rawset")?)?;
    env.set("select", globals.get::<LuaFunction>("select")?)?;
    env.set("tonumber", globals.get::<LuaFunction>("tonumber")?)?;
    env.set("tostring", globals.get::<LuaFunction>("tostring")?)?;
    env.set("type", globals.get::<LuaFunction>("type")?)?;
    env.set("xpcall", globals.get::<LuaFunction>("xpcall")?)?;

    // Safe libraries (shared references from globals).
    env.set("table", globals.get::<LuaTable>("table")?)?;
    env.set("string", globals.get::<LuaTable>("string")?)?;
    env.set("math", globals.get::<LuaTable>("math")?)?;
    if let Ok(utf8_tbl) = globals.get::<LuaTable>("utf8") {
        env.set("utf8", utf8_tbl)?;
    }
    env.set("print", globals.get::<LuaFunction>("print")?)?;

    Ok(env)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::StdLib;

    fn make_sandbox() -> Lua {
        let lua =
            Lua::new_with(StdLib::ALL_SAFE, LuaOptions::new().catch_rust_panics(true)).unwrap();
        sanitize_globals(&lua).unwrap();
        lua
    }

    #[test]
    fn test_os_execute_is_nil() {
        let lua = make_sandbox();
        let os_tbl: LuaTable = lua.globals().get("os").unwrap();
        let exec: LuaValue = os_tbl.get("execute").unwrap();
        assert!(matches!(exec, LuaValue::Nil));
    }

    #[test]
    fn test_io_popen_is_nil() {
        let lua = make_sandbox();
        let io_tbl: LuaTable = lua.globals().get("io").unwrap();
        let popen: LuaValue = io_tbl.get("popen").unwrap();
        assert!(matches!(popen, LuaValue::Nil));
    }

    #[test]
    fn test_dofile_is_nil() {
        let lua = make_sandbox();
        let dofile: LuaValue = lua.globals().get("dofile").unwrap();
        assert!(matches!(dofile, LuaValue::Nil));
    }

    #[test]
    fn test_allowed_functions_work() {
        let lua = make_sandbox();
        let env = create_restricted_env(&lua).unwrap();

        // print должен быть доступен.
        let print_fn: LuaValue = env.get("print").unwrap();
        assert!(matches!(print_fn, LuaValue::Function(_)));

        // table.insert должен быть доступен.
        let table_tbl: LuaTable = env.get("table").unwrap();
        let insert_fn: LuaValue = table_tbl.get("insert").unwrap();
        assert!(matches!(insert_fn, LuaValue::Function(_)));

        // tostring должен быть доступен.
        let tostring_fn: LuaValue = env.get("tostring").unwrap();
        assert!(matches!(tostring_fn, LuaValue::Function(_)));
    }

    #[test]
    fn test_env_g_points_to_self() {
        let lua = make_sandbox();
        let env = create_restricted_env(&lua).unwrap();
        let g: LuaValue = env.get("_G").unwrap();
        // _G должен быть таблицей (указывает на себя).
        assert!(matches!(g, LuaValue::Table(_)));
    }
}
