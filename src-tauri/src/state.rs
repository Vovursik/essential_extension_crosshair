use std::sync::atomic::{AtomicBool, AtomicUsize};

pub struct AppState {
    pub toggle_mode: AtomicBool,
    pub press_count_show: AtomicUsize,
    pub press_count_hide: AtomicUsize,
    pub press_counter: AtomicUsize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            toggle_mode: AtomicBool::new(false),
            press_count_show: AtomicUsize::new(1),
            press_count_hide: AtomicUsize::new(1),
            press_counter: AtomicUsize::new(0),
        }
    }
}