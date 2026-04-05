use mlua::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use smithay_client_toolkit::reexports::calloop::channel::Sender;
use tracing::{debug, error, warn};

/// Message sent by timer threads to the main loop.
pub struct TimerFire {
    #[allow(dead_code)]
    pub aura_id: String,
    pub timer_id: u64,
}

/// Timer registry with aura tracking and calloop-compatible channel.
///
/// Timer threads send `TimerFire` through a calloop channel. The main loop
/// receives these via the calloop event loop and calls the Lua callbacks in
/// the main thread.
#[derive(Clone)]
pub struct TimerRegistry {
    next_id: Arc<AtomicU64>,
    timers: Arc<Mutex<HashMap<u64, ActiveTimer>>>,
    aura_timers: Arc<Mutex<HashMap<String, Vec<u64>>>>,
    callbacks: Arc<Mutex<HashMap<u64, LuaFunction>>>,
    fire_tx: Sender<TimerFire>,
}

impl TimerRegistry {
    pub fn new(fire_tx: Sender<TimerFire>) -> Self {
        Self {
            next_id: Arc::new(AtomicU64::new(1)),
            timers: Arc::new(Mutex::new(HashMap::new())),
            aura_timers: Arc::new(Mutex::new(HashMap::new())),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            fire_tx,
        }
    }

    /// Lock a mutex, recovering from poisoning.
    fn lock<'a, T>(lock: &'a Mutex<T>, context: &str) -> std::sync::MutexGuard<'a, T> {
        match lock.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                error!("Mutex poisoned in {}: {}", context, poisoned);
                poisoned.into_inner()
            }
        }
    }

    /// Process a single timer fire — call the Lua callback.
    /// Called from the calloop event loop channel callback.
    pub fn process_single_fire(&self, fire: TimerFire) {
        let cb_opt = {
            let cbs = Self::lock(&self.callbacks, "process_single_fire");
            cbs.get(&fire.timer_id).cloned()
        };

        match cb_opt {
            Some(cb) => {
                if let Err(e) = cb.call::<()>(()) {
                    warn!("Timer {} callback error: {}", fire.timer_id, e);
                }
            }
            None => {
                debug!("Timer {} callback not found (already cancelled?)", fire.timer_id);
            }
        }
    }

    /// Register a timer that fires every `ms` milliseconds.
    pub fn request_interval_for_aura(&self, aura_id: &str, ms: u32, cb: LuaFunction) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_for_thread = stop_flag.clone();
        let tx = self.fire_tx.clone();
        let aura = aura_id.to_string();

        {
            let mut cbs = Self::lock(&self.callbacks, "request_interval");
            cbs.insert(id, cb);
        }

        let timer = ActiveTimer { id, stop_flag: stop_flag.clone() };
        {
            let mut timers = Self::lock(&self.timers, "request_interval_timers");
            timers.insert(id, timer);
        }

        let fire_aura = aura.clone();
        {
            let mut aura_timers = Self::lock(&self.aura_timers, "request_interval_aura");
            aura_timers.entry(aura).or_default().push(id);
        }

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

        {
            let mut cbs = Self::lock(&self.callbacks, "request_timeout");
            cbs.insert(id, cb);
        }

        let timer = ActiveTimer { id, stop_flag: stop_flag.clone() };
        {
            let mut timers = Self::lock(&self.timers, "request_timeout_timers");
            timers.insert(id, timer);
        }

        let fire_aura = aura.clone();
        {
            let mut aura_timers = Self::lock(&self.aura_timers, "request_timeout_aura");
            aura_timers.entry(aura).or_default().push(id);
        }

        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(ms as u64));
            if !stop_for_thread.load(Ordering::Relaxed) {
                if tx.send(TimerFire { timer_id: id, aura_id: fire_aura }).is_err() {
                    debug!("Timer {} send failed — channel disconnected", id);
                }
            }
        });

        id
    }

    pub fn cancel(&self, id: u64) -> bool {
        if let Some(timer) = Self::lock(&self.timers, "cancel").remove(&id) {
            timer.stop_flag.store(true, Ordering::Relaxed);
        }
        Self::lock(&self.callbacks, "cancel_cb").remove(&id);
        {
            let mut aura_timers: std::sync::MutexGuard<'_, HashMap<String, Vec<u64>>> =
                Self::lock(&self.aura_timers, "cancel_aura");
            for timers in aura_timers.values_mut() {
                timers.retain(|t| *t != id);
            }
        }
        true
    }

    pub fn cancel_all_for_aura(&self, aura_id: &str) {
        let timer_ids: Option<Vec<u64>> = {
            let mut aura_timers = Self::lock(&self.aura_timers, "cancel_all_aura");
            aura_timers.remove(aura_id)
        };
        let Some(timer_ids) = timer_ids else { return };

        // Stop timers and collect callback IDs.
        let callback_ids: Vec<u64> = {
            let mut timers = Self::lock(&self.timers, "cancel_all_timers");
            let mut ids = Vec::new();
            for id in &timer_ids {
                if let Some(timer) = timers.remove(id) {
                    timer.stop_flag.store(true, Ordering::Relaxed);
                    ids.push(*id);
                }
            }
            ids
        };

        // Remove callbacks — memory leak fix.
        {
            let mut cbs = Self::lock(&self.callbacks, "cancel_all_callbacks");
            for id in callback_ids {
                cbs.remove(&id);
            }
        }
    }

    pub fn cancel_all(&self) {
        let mut aura_timers = Self::lock(&self.aura_timers, "cancel_all");
        let mut timers = Self::lock(&self.timers, "cancel_all_timers");
        for (_aura_id, timer_ids) in aura_timers.drain() {
            for id in timer_ids {
                if let Some(timer) = timers.remove(&id) {
                    timer.stop_flag.store(true, Ordering::Relaxed);
                }
            }
        }
        Self::lock(&self.callbacks, "cancel_all_callbacks").clear();
    }
}

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
