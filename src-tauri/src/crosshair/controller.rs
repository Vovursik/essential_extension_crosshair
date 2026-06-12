use tauri::AppHandle;

use crate::{crosshair::loader, crosshair::sender, storage};

pub fn load_from_user(app: &AppHandle, path: String) {
    if let Some(data) = loader::load_zip(&path) {
        sender::send(app, data);

        storage::save_path(&path);
    }
}

pub fn restore_on_start(app: &AppHandle) {
    if let Some(path) = storage::load_path() {
        if let Some(data) = loader::load_zip(&path) {
            sender::send(app, data);
        }
    }
}