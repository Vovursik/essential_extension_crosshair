use std::sync::{atomic::AtomicBool};

pub struct AppState {
    pub crosshair_path: std::sync::Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            crosshair_path: std::sync::Mutex::new(None),
        }
    }
}