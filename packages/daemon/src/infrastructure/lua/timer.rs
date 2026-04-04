use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A single active timer.
struct ActiveTimer {
    #[allow(dead_code)]
    id: u64,
    stop_flag: Arc<AtomicBool>,
}

/// Registry for Lua timers with cancellable threads, organized by aura.
///
/// Each timer is associated with an aura_id. When an aura is unloaded,
/// all its timers are cancelled and their threads exit cleanly.
#[derive(Clone, Default)]
pub struct TimerRegistry {
    next_id: Arc<AtomicU64>,
    /// Timer ID → ActiveTimer (for cancellation by ID).
    timers: Arc<Mutex<HashMap<u64, ActiveTimer>>>,
    /// Aura ID → list of timer IDs (for bulk cancellation).
    aura_timers: Arc<Mutex<HashMap<String, Vec<u64>>>>,
}

impl TimerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Request a new interval timer for a specific aura. Returns the timer ID.
    /// A thread calls the Lua callback every `ms` milliseconds.
    /// The thread exits when `cancel(id)` or `cancel_all_for_aura(aura_id)` is called.
    pub fn request_interval_for_aura(&self, aura_id: &str, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();

        // Store timer info for cancellation.
        let timer = ActiveTimer {
            id,
            stop_flag: stop_flag.clone(),
        };
        self.timers.lock().unwrap().insert(id, timer);

        // Associate timer with aura.
        self.aura_timers
            .lock()
            .unwrap()
            .entry(aura_id.to_string())
            .or_default()
            .push(id);

        // Spawn thread that calls the callback periodically.
        std::thread::spawn(move || {
            let interval = Duration::from_millis(ms as u64);
            loop {
                std::thread::sleep(interval);
                if stop_for_thread.load(Ordering::Relaxed) {
                    break;
                }
                let _ = cb.call::<()>(());
            }
        });

        id
    }

    /// Request a one-shot timeout timer for a specific aura.
    pub fn request_timeout_for_aura(&self, aura_id: &str, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();

        let timer = ActiveTimer {
            id,
            stop_flag: stop_flag.clone(),
        };
        self.timers.lock().unwrap().insert(id, timer);

        self.aura_timers
            .lock()
            .unwrap()
            .entry(aura_id.to_string())
            .or_default()
            .push(id);

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(ms as u64));
            if !stop_for_thread.load(Ordering::Relaxed) {
                let _ = cb.call::<()>(());
            }
        });

        id
    }

    /// Cancel a specific timer by ID.
    pub fn cancel(&self, id: u64) -> bool {
        if let Some(timer) = self.timers.lock().unwrap().remove(&id) {
            timer.stop_flag.store(true, Ordering::Relaxed);
            // Also remove from aura_timers.
            let mut aura_timers = self.aura_timers.lock().unwrap();
            for timers in aura_timers.values_mut() {
                timers.retain(|t| *t != id);
            }
            true
        } else {
            false
        }
    }

    /// Cancel all timers for a specific aura. Stops all associated timer threads.
    pub fn cancel_all_for_aura(&self, aura_id: &str) {
        let mut aura_timers = self.aura_timers.lock().unwrap();
        if let Some(timer_ids) = aura_timers.remove(aura_id) {
            let mut timers = self.timers.lock().unwrap();
            for id in timer_ids {
                if let Some(timer) = timers.remove(&id) {
                    timer.stop_flag.store(true, Ordering::Relaxed);
                }
            }
        }
    }

    /// Cancel all timers across all auras.
    pub fn cancel_all(&self) {
        let mut aura_timers = self.aura_timers.lock().unwrap();
        let mut timers = self.timers.lock().unwrap();
        for (_aura_id, timer_ids) in aura_timers.drain() {
            for id in timer_ids {
                if let Some(timer) = timers.remove(&id) {
                    timer.stop_flag.store(true, Ordering::Relaxed);
                }
            }
        }
    }
}

/// Creates the Lua `waio.timer` module.
///
/// The `aura_id` is baked into the module so that all timers created
/// from this Lua context are associated with the correct aura.
pub fn create_module(
    lua: &Lua,
    registry: TimerRegistry,
    aura_id: String,
) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let reg_for_interval = registry.clone();
    let aura_for_interval = aura_id.clone();
    m.set(
        "interval",
        lua.create_function(move |_, (ms, cb): (u32, LuaFunction)| {
            let id = reg_for_interval.request_interval_for_aura(&aura_for_interval, ms, cb);
            Ok(id)
        })?,
    )?;

    let reg_for_timeout = registry.clone();
    let aura_for_timeout = aura_id;
    m.set(
        "timeout",
        lua.create_function(move |_, (ms, cb): (u32, LuaFunction)| {
            let id = reg_for_timeout.request_timeout_for_aura(&aura_for_timeout, ms, cb);
            Ok(id)
        })?,
    )?;

    let reg_for_cancel = registry;
    m.set(
        "cancel",
        lua.create_function(move |_, id: u64| {
            Ok(reg_for_cancel.cancel(id))
        })?,
    )?;

    Ok(m)
}
