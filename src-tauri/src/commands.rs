use tauri::{AppHandle, Emitter};
use serde_json::json;
use std::sync::Arc;

use crate::state::{AppState, Crosshair};
use crate::storage;

pub fn restore_crosshair(app: &AppHandle, _state: &Arc<AppState>) {
    if let Some((html, css)) = storage::load_active() {
        let _ = app.emit_to("main", "set-crosshair", json!({ "html": html, "css": css }));
    }
}

pub async fn load_crosshair(app: AppHandle, state: Arc<AppState>) -> Result<(), String> {
    use tauri_plugin_dialog::DialogExt;

    let file = app.dialog().file()
        .add_filter("", &["zip"])
        .blocking_pick_file();

    let Some(file) = file else { return Ok(()) };
    let zip_path = file.to_string();

    let dir = storage::extract_zip(&zip_path).map_err(|e| e.to_string())?;
    let dir_str = dir.to_string_lossy().to_string();

    _apply_crosshair(&app, &state, &dir_str)
}

fn _apply_crosshair(app: &AppHandle, state: &Arc<AppState>, dir: &str) -> Result<(), String> {
    let (html, css) = storage::read(dir).map_err(|e| e.to_string())?;

    *state.active.lock().unwrap() = Some(Crosshair {
        name: dir.to_string(),
        html: html.clone(),
        css: css.clone(),
    });

    storage::save_active(&html, &css);

    app.emit_to("main", "set-crosshair", json!({ "html": html, "css": css }))
        .map_err(|e| e.to_string())
}