use std::sync::{Arc, Mutex};
use tracing::warn;

/// Maximum pending property updates before warning.
/// Prevents unbounded memory growth if the main loop lags.
const COMMAND_QUEUE_WARN_THRESHOLD: usize = 1000;

pub struct PropertyUpdate {
    pub aura_id: String,
    pub property: String,
    pub value: String,
}

pub type CommandQueue = Arc<Mutex<Vec<PropertyUpdate>>>;

pub fn new_command_queue() -> CommandQueue {
    Arc::new(Mutex::new(Vec::new()))
}

/// Push a property update to the command queue.
/// Logs a warning if the queue exceeds the threshold.
pub fn push_command(queue: &CommandQueue, update: PropertyUpdate) {
    let mut q = match queue.lock() {
        Ok(g) => g,
        Err(poisoned) => {
            warn!("CommandQueue mutex poisoned: {}", poisoned);
            poisoned.into_inner()
        }
    };

    if q.len() >= COMMAND_QUEUE_WARN_THRESHOLD {
        warn!(
            "CommandQueue size ({}) exceeds threshold ({}). Possible render lag.",
            q.len(),
            COMMAND_QUEUE_WARN_THRESHOLD
        );
    }

    q.push(update);
}
