use rdev::{listen, Button, EventType};
use tauri::{AppHandle, Emitter};

pub fn start_mouse(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let _ = listen(move |event| match event.event_type {
            EventType::ButtonPress(Button::Right) => {
                let _ = app_handle.emit("mouse-state", true);
            }
            EventType::ButtonRelease(Button::Right) => {
                let _ = app_handle.emit("mouse-state", false);
            }
            _ => {}
        });
    });
}