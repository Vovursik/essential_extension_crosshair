use std::io::{Cursor, Read};
use tauri::{AppHandle, Emitter};
use zip::ZipArchive;

use crate::storage;

struct CrosshairData {
    html: String,
    css: String,
}

fn load_zip(path: &str) -> Option<CrosshairData> {
    let bytes = std::fs::read(path).ok()?;
    let mut archive = ZipArchive::new(Cursor::new(bytes)).ok()?;

    let mut html = String::new();
    let mut css = String::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).ok()?;
        let name = file.name().to_string();

        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            continue;
        }

        match name.as_str() {
            n if n.ends_with("index.html") => html = content,
            n if n.ends_with("style.css") => css = content,
            _ => {}
        }
    }

    Some(CrosshairData { html, css })
}

fn send(app: &AppHandle, data: CrosshairData) {
    let payload = serde_json::json!({ "html": data.html, "css": data.css });
    let _ = app.emit_to("main", "set-crosshair", payload);
}

pub fn load_from_user(app: &AppHandle, path: String) {
    if let Some(data) = load_zip(&path) {
        send(app, data);
        storage::save_path(&path);
    }
}

pub fn restore_on_start(app: &AppHandle) {
    if let Some(path) = storage::load_path() {
        if let Some(data) = load_zip(&path) {
            send(app, data);
        }
    }
}