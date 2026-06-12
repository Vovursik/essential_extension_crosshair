use std::sync::{atomic::AtomicBool};

pub struct AppState {
    pub toggle_mode: AtomicBool,
    pub crosshair_path: std::sync::Mutex<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            toggle_mode: AtomicBool::new(false),
            crosshair_path: std::sync::Mutex::new(None),
        }
    }
}