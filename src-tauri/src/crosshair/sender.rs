use tauri::{AppHandle, Emitter};

use crate::crosshair::loader::CrosshairData;

pub fn send(app: &AppHandle, data: CrosshairData) {
    let payload = serde_json::json!({
        "html": data.html,
        "css": data.css
    });

    let _ = app.emit_to("main", "set-crosshair", payload);
}