use std::sync::{Arc, atomic::Ordering};
use serde::Serialize;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

use crate::state::AppState;
use crate::storage::{self, CrosshairMeta};
use crate::crosshair::{self, controller};

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

#[tauri::command]
pub fn get_press_count_show(state: State<Arc<AppState>>) -> usize {
    state.press_count_show.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn set_press_count_show(state: State<Arc<AppState>>, count: usize) {
    let clamped = count.max(1);
    state.press_count_show.store(clamped, Ordering::Relaxed);
    storage::save_press_count_show(clamped);
}

#[tauri::command]
pub fn get_press_count_hide(state: State<Arc<AppState>>) -> usize {
    state.press_count_hide.load(Ordering::Relaxed)
}

#[tauri::command]
pub fn set_press_count_hide(state: State<Arc<AppState>>, count: usize) {
    let clamped = count.max(1);
    state.press_count_hide.store(clamped, Ordering::Relaxed);
    storage::save_press_count_hide(clamped);
}

#[tauri::command]
pub fn list_crosshairs() -> Vec<CrosshairMeta> {
    storage::list_crosshairs()
}

#[tauri::command]
pub fn get_active_crosshair() -> Option<String> {
    storage::get_active()
}

#[derive(Serialize)]
pub struct CrosshairPreview {
    html: String,
    css: String,
}

#[tauri::command]
pub fn get_crosshair_preview(id: String) -> Option<CrosshairPreview> {
    storage::read_crosshair(&id).map(|(html, css)| CrosshairPreview { html, css })
}

#[tauri::command]
pub fn apply_crosshair(app: AppHandle, id: String) {
    controller::apply_crosshair(&app, &id);
}

#[tauri::command]
pub fn delete_crosshair(app: AppHandle, id: String) {
    controller::delete_crosshair(&app, &id);
}