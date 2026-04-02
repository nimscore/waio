use std::sync::{Arc, Mutex};

pub struct PropertyUpdate {
    pub aura_id: String,
    pub property: String,
    pub value: String,
}

pub type CommandQueue = Arc<Mutex<Vec<PropertyUpdate>>>;

pub fn new_command_queue() -> CommandQueue {
    Arc::new(Mutex::new(Vec::new()))
}
