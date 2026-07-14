#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod tray;
mod mouse;
mod crosshair;
mod storage;
mod window;
mod commands;

use std::sync::Arc;
use std::sync::atomic::Ordering;

use state::AppState;
use mouse::start_mouse;
use tray::create_tray;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::toggle_mode,
            commands::get_mode,
            commands::load_crosshair,
            commands::list_crosshairs,
            commands::get_active_crosshair,
            commands::get_crosshair_preview,
            commands::apply_crosshair,
            commands::delete_crosshair,
            commands::get_press_count_show,
            commands::set_press_count_show,
            commands::get_press_count_hide,
            commands::set_press_count_hide
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            let monitor = window.current_monitor().unwrap().unwrap();
            let screen = monitor.size();

            let win_size = 200;

            let x = (screen.width as i32 - win_size) / 2;
            let y = (screen.height as i32 - win_size) / 2;

            window.set_position(tauri::PhysicalPosition::new(x, y)).ok();
            window.set_shadow(false).ok();

            let handle = app.handle().clone();

            let state = Arc::new(AppState::new());
            state.press_count_show.store(storage::get_press_count_show(), Ordering::Relaxed);
            state.press_count_hide.store(storage::get_press_count_hide(), Ordering::Relaxed);
            app.manage(state.clone());
            app.get_webview_window("main")
                .unwrap()
                .set_ignore_cursor_events(true)
                .unwrap();

            create_tray(&handle);
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