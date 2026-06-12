use rdev::{listen, Button, EventType};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tauri::{Emitter, AppHandle};

use crate::state::AppState;

pub fn start_mouse(app_handle: AppHandle, state: Arc<AppState>) {
    std::thread::spawn(move || {
        let _ = listen(move |event| {
            let toggle = state.toggle_mode.load(Ordering::Relaxed);

            match event.event_type {

                EventType::ButtonPress(Button::Right) => {
                    if toggle {
                        static ACTIVE: AtomicBool = AtomicBool::new(false);

                        let new = !ACTIVE.load(Ordering::Relaxed);
                        ACTIVE.store(new, Ordering::Relaxed);

                        let _ = app_handle.emit("mouse-state", new);
                    } else {
                        let _ = app_handle.emit("mouse-state", true);
                    }
                }

                EventType::ButtonRelease(Button::Right) => {
                    if !toggle {
                        let _ = app_handle.emit("mouse-state", false);
                    }
                }

                _ => {}
            }
        });
    });
}