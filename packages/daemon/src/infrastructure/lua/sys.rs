use mlua::prelude::*;
use mlua::Nil;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn create_module(lua: &Lua) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    m.set(
        "battery",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            let level = read_battery_capacity().unwrap_or(-1.0);
            let charging = read_battery_status().unwrap_or(false);
            let time_remaining = read_battery_time_remaining();

            let r = lua.create_table()?;
            r.set("level", level)?;
            r.set("charging", charging)?;
            match time_remaining {
                Some(t) => r.set("time_remaining", t)?,
                None => r.set("time_remaining", Nil)?,
            }
            Ok(r)
        })?,
    )?;

    m.set(
        "cpu_usage",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            use sysinfo::System;
            let mut sys = System::new_all();
            thread::sleep(Duration::from_millis(50));
            sys.refresh_cpu_usage();

            let global = sys.global_cpu_usage();
            let cores = sys
                .cpus()
                .iter()
                .map(|c| c.cpu_usage() as f64)
                .collect::<Vec<_>>();

            let r = lua.create_table()?;
            r.set("global", global as f64)?;
            r.set("cores", cores)?;
            Ok(r)
        })?,
    )?;

    m.set(
        "memory",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            use sysinfo::System;
            let sys = System::new_all();
            let used = sys.used_memory();
            let total = sys.total_memory();
            let percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            let r = lua.create_table()?;
            r.set("used", used)?;
            r.set("total", total)?;
            r.set("percent", percent)?;
            Ok(r)
        })?,
    )?;

    m.set(
        "temperatures",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            use sysinfo::Components;
            let components = Components::new_with_refreshed_list();

            let temps = lua.create_table()?;
            let mut idx: i64 = 1;
            for comp in components.iter() {
                if let Some(temp) = comp.temperature() {
                    let entry = lua.create_table()?;
                    entry.set("name", comp.label())?;
                    entry.set("temp", temp as f64)?;
                    entry.set("max", comp.max().unwrap_or(0.0) as f64)?;
                    temps.set(idx, entry)?;
                    idx += 1;
                }
            }
            Ok(temps)
        })?,
    )?;

    m.set(
        "disks",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            use sysinfo::Disks;
            let disks = Disks::new_with_refreshed_list();

            let arr = lua.create_table()?;
            let mut idx: i64 = 1;
            for disk in disks.iter() {
                let entry = lua.create_table()?;
                let mount = disk.mount_point().to_string_lossy().to_string();
                let fs = disk.file_system().to_string_lossy().to_string();
                entry.set("mount", mount)?;
                entry.set(
                    "used",
                    disk.total_space().saturating_sub(disk.available_space()),
                )?;
                entry.set("total", disk.total_space())?;
                entry.set("filesystem", fs)?;
                arr.set(idx, entry)?;
                idx += 1;
            }
            Ok(arr)
        })?,
    )?;

    m.set(
        "uptime",
        lua.create_function(|_, ()| -> LuaResult<u64> {
            use sysinfo::System;
            Ok(System::uptime())
        })?,
    )?;

    m.set(
        "network",
        lua.create_function(|lua, ()| -> LuaResult<LuaTable> {
            use sysinfo::Networks;
            let networks = Networks::new_with_refreshed_list();

            let mut rx: u64 = 0;
            let mut tx: u64 = 0;
            for (_, data) in networks.iter() {
                rx += data.total_received();
                tx += data.total_transmitted();
            }

            let r = lua.create_table()?;
            r.set("rx", rx)?;
            r.set("tx", tx)?;
            Ok(r)
        })?,
    )?;

    Ok(m)
}

// --- Battery helpers via /sys/class/power_supply ---

fn read_battery_capacity() -> Option<f64> {
    let path = Path::new("/sys/class/power_supply/BAT0/capacity");
    fs::read_to_string(path).ok()?.trim().parse::<f64>().ok()
}

fn read_battery_status() -> Option<bool> {
    let path = Path::new("/sys/class/power_supply/BAT0/status");
    let status = fs::read_to_string(path).ok()?;
    let status = status.trim().to_lowercase();
    Some(status == "charging" || status == "full")
}

fn read_battery_time_remaining() -> Option<f64> {
    // Try to compute time remaining from energy_now and power_now (watt-hours and watts).
    let energy_path = Path::new("/sys/class/power_supply/BAT0/energy_now");
    let power_path = Path::new("/sys/class/power_supply/BAT0/power_now");

    if let (Ok(e), Ok(p)) = (
        fs::read_to_string(energy_path),
        fs::read_to_string(power_path),
    ) {
        let energy: f64 = e.trim().parse().ok()?;
        let power: f64 = p.trim().parse().ok()?;
        if power > 0.0 {
            // energy in Wh, power in W -> hours -> seconds
            return Some((energy / power) * 3600.0);
        }
    }

    // Fallback: try time_to_empty_now / time_to_full_now (microseconds).
    for file in &["time_to_empty_now", "time_to_full_now"] {
        let path = Path::new("/sys/class/power_supply/BAT0").join(file);
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(micros) = content.trim().parse::<f64>() {
                if micros > 0.0 {
                    return Some(micros / 1_000_000.0);
                }
            }
        }
    }

    None
}
