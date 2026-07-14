use rdev::{listen, Button, EventType};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tauri::{Emitter, AppHandle};

use crate::state::AppState;

pub fn start_mouse(app_handle: AppHandle, state: Arc<AppState>) {
    std::thread::spawn(move || {
        static ACTIVE: AtomicBool = AtomicBool::new(false);

        let _ = listen(move |event| {
            let toggle = state.toggle_mode.load(Ordering::Relaxed);

            match event.event_type {
                EventType::ButtonPress(Button::Right) => {
                    if toggle {
                        let active_now = ACTIVE.load(Ordering::Relaxed);

                        let target = if active_now {
                            state.press_count_hide.load(Ordering::Relaxed).max(1)
                        } else {
                            state.press_count_show.load(Ordering::Relaxed).max(1)
                        };

                        let count = state.press_counter.fetch_add(1, Ordering::Relaxed) + 1;

                        if count >= target {
                            state.press_counter.store(0, Ordering::Relaxed);

                            let new = !active_now;
                            ACTIVE.store(new, Ordering::Relaxed);

                            let _ = app_handle.emit("mouse-state", new);
                        }
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