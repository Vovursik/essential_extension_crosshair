#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod tray;
mod mouse;
mod crosshair;
mod storage;

use std::sync::Arc;

use state::AppState;
use mouse::start_mouse;
use tray::create_tray;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle().clone();

            let state = Arc::new(AppState::new());
            app.manage(state.clone());
            app.get_webview_window("main")
                .unwrap()
                .set_ignore_cursor_events(true)
                .unwrap();

            // tray
            create_tray(&handle);

            // mouse listener
            start_mouse(handle.clone(), state.clone());

std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_millis(500));
    crosshair::restore_on_start(&handle);
});
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running app");
}