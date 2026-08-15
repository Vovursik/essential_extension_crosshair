#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;
mod mouse;
mod crosshair;
mod storage;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            let monitor = window.current_monitor().unwrap().unwrap();
            let screen = monitor.size();
            let win_size = 200;

            let x = (screen.width as i32 - win_size) / 2;
            let y = (screen.height as i32 - win_size) / 2;

            window.set_position(tauri::PhysicalPosition::new(x, y)).ok();
            window.set_shadow(false).ok();
            window.set_ignore_cursor_events(true).ok();

            let handle = app.handle().clone();

            tray::create_tray(&handle);
            mouse::start_mouse(handle.clone());

            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(500));
                crosshair::restore_on_start(&handle);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running app");
}