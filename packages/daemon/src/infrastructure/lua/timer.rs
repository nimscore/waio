use mlua::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A single active timer.
struct ActiveTimer {
    id: u64,
    stop_flag: Arc<AtomicBool>,
}

/// Registry for Lua timers with cancellable threads.
///
/// Each timer spawns a thread that calls the Lua callback periodically.
/// Threads check a stop_flag each iteration and exit when the timer is cancelled.
#[derive(Clone, Default)]
pub struct TimerRegistry {
    next_id: Arc<AtomicU64>,
    active: Arc<Mutex<Vec<ActiveTimer>>>,
}

impl TimerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Request a new interval timer. Returns the timer ID.
    /// A thread calls the Lua callback every `ms` milliseconds.
    /// The thread exits when `cancel(id)` is called.
    pub fn request_interval(&self, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();

        // Store timer info for cancellation.
        self.active.lock().unwrap().push(ActiveTimer {
            id,
            stop_flag: stop_flag.clone(),
        });

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

    /// Request a one-shot timeout timer. Returns the timer ID.
    pub fn request_timeout(&self, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();

        self.active.lock().unwrap().push(ActiveTimer {
            id,
            stop_flag,
        });

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(ms as u64));
            if !stop_for_thread.load(Ordering::Relaxed) {
                let _ = cb.call::<()>(());
            }
        });

        id
    }

    /// Cancel a timer by ID. Sets the stop flag to halt the thread.
    pub fn cancel(&self, id: u64) -> bool {
        let mut active = self.active.lock().unwrap();
        if let Some(idx) = active.iter().position(|t| t.id == id) {
            let timer = active.remove(idx);
            timer.stop_flag.store(true, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Cancel all timers. Stops all timer threads.
    pub fn cancel_all(&self) {
        let mut active = self.active.lock().unwrap();
        for timer in active.drain(..) {
            timer.stop_flag.store(true, Ordering::Relaxed);
        }
    }
}

/// Creates the Lua `waio.timer` module.
pub fn create_module(lua: &Lua, registry: TimerRegistry) -> LuaResult<LuaTable> {
    let m = lua.create_table()?;

    let reg_for_interval = registry.clone();
    m.set(
        "interval",
        lua.create_function(move |_, (ms, cb): (u32, LuaFunction)| {
            let id = reg_for_interval.request_interval(ms, cb);
            Ok(id)
        })?,
    )?;

    let reg_for_timeout = registry.clone();
    m.set(
        "timeout",
        lua.create_function(move |_, (ms, cb): (u32, LuaFunction)| {
            let id = reg_for_timeout.request_timeout(ms, cb);
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
