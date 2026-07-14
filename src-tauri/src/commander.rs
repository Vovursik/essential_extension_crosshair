use std::sync::{Arc, atomic::Ordering};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::state::AppState;
use crate::crosshair;

#[tauri::command]
pub fn toggle_mode(state: State<Arc<AppState>>) -> bool {
    let new = !state.toggle_mode.load(Ordering::Relaxed);
    state.toggle_mode.store(new, Ordering::Relaxed);
    new
}

#[tauri::command]
pub fn get_mode(state: State<Arc<AppState>>) -> bool {
    state.toggle_mode.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn load_crosshair(app: AppHandle) {
    let app_clone = app.clone();

    app.dialog()
        .file()
        .add_filter("zip", &["zip"])
        .pick_file(move |file| {
            if let Some(path) = file {
                let path_buf = match path {
                    tauri_plugin_dialog::FilePath::Path(p) => p,
                    _ => return,
                };
                let path_str = path_buf.to_string_lossy().to_string();
                crosshair::load_from_user(&app_clone, path_str);
            }
        });
}