use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Message sent by timer threads to the main loop.
pub struct TimerFire {
    #[allow(dead_code)]
    pub aura_id: String,
    pub timer_id: u64,
}

/// Timer registry with aura tracking and calloop-compatible fire messages.
///
/// Timer threads send `TimerFire` through a crossbeam channel. The main loop
/// receives these and calls the Lua callbacks in the main thread.
#[derive(Clone)]
pub struct TimerRegistry {
    next_id: Arc<AtomicU64>,
    /// Timer ID → ActiveTimer (for cancellation by ID).
    timers: Arc<Mutex<HashMap<u64, ActiveTimer>>>,
    /// Aura ID → list of timer IDs (for bulk cancellation).
    aura_timers: Arc<Mutex<HashMap<String, Vec<u64>>>>,
    /// Lua callbacks keyed by timer ID.
    callbacks: Arc<Mutex<HashMap<u64, LuaFunction>>>,
    /// Channel for timer fire messages. Main loop polls this.
    pub fire_tx: crossbeam_channel::Sender<TimerFire>,
    pub fire_rx: crossbeam_channel::Receiver<TimerFire>,
}

impl TimerRegistry {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Self {
            next_id: Arc::new(AtomicU64::new(1)),
            timers: Arc::new(Mutex::new(HashMap::new())),
            aura_timers: Arc::new(Mutex::new(HashMap::new())),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            fire_tx: tx,
            fire_rx: rx,
        }
    }

    /// Process all pending timer fires. Call this from the main event loop.
    /// Calls Lua callbacks in the main thread (safe).
    pub fn process_fires(&self) {
        while let Ok(fire) = self.fire_rx.try_recv() {
            let cbs = self.callbacks.lock().unwrap();
            if let Some(cb) = cbs.get(&fire.timer_id) {
                let _ = cb.call::<()>(());
            }
        }
    }

    /// Register a timer that fires every `ms` milliseconds.
    /// The callback is called from the main thread via the fire channel.
    pub fn request_interval_for_aura(&self, aura_id: &str, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();
        let tx = self.fire_tx.clone();
        let aura = aura_id.to_string();

        // Store callback.
        self.callbacks.lock().unwrap().insert(id, cb);

        // Store timer info for cancellation.
        let timer = ActiveTimer {
            id,
            stop_flag: stop_flag.clone(),
        };
        self.timers.lock().unwrap().insert(id, timer);

        // Associate timer with aura.
        let aura_for_entry = aura.clone();
        let fire_aura = aura.clone();
        self.aura_timers
            .lock()
            .unwrap()
            .entry(aura_for_entry)
            .or_default()
            .push(id);

        // Spawn thread that sends fire messages.
        std::thread::spawn(move || {
            let interval = Duration::from_millis(ms as u64);
            loop {
                std::thread::sleep(interval);
                if stop_for_thread.load(Ordering::Relaxed) {
                    break;
                }
                if tx.send(TimerFire { timer_id: id, aura_id: fire_aura.clone() }).is_err() {
                    break;
                }
            }
        });

        id
    }

    pub fn request_timeout_for_aura(&self, aura_id: &str, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();
        let tx = self.fire_tx.clone();
        let aura = aura_id.to_string();

        self.callbacks.lock().unwrap().insert(id, cb);

        let timer = ActiveTimer {
            id,
            stop_flag: stop_flag.clone(),
        };
        self.timers.lock().unwrap().insert(id, timer);

        let aura_for_entry = aura.clone();
        let fire_aura = aura.clone();
        self.aura_timers
            .lock()
            .unwrap()
            .entry(aura_for_entry)
            .or_default()
            .push(id);

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(ms as u64));
            if !stop_for_thread.load(Ordering::Relaxed) {
                let _ = tx.send(TimerFire { timer_id: id, aura_id: fire_aura });
            }
        });

        id
    }

    pub fn cancel(&self, id: u64) -> bool {
        if let Some(timer) = self.timers.lock().unwrap().remove(&id) {
            timer.stop_flag.store(true, Ordering::Relaxed);
        }
        self.callbacks.lock().unwrap().remove(&id);
        let mut aura_timers = self.aura_timers.lock().unwrap();
        for timers in aura_timers.values_mut() {
            timers.retain(|t| *t != id);
        }
        true
    }

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
        self.callbacks.lock().unwrap().retain(|_, _| {
            // Keep all callbacks — they're cleaned up on aura removal.
            true
        });
    }

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
        self.callbacks.lock().unwrap().clear();
    }
}

/// A single active timer.
struct ActiveTimer {
    #[allow(dead_code)]
    id: u64,
    stop_flag: Arc<AtomicBool>,
}

/// Creates the Lua `waio.timer` module.
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
