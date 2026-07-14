use std::path::Path;
use tauri::AppHandle;

use crate::{crosshair::loader, crosshair::loader::CrosshairData, crosshair::sender, storage};

/// Пользователь выбрал zip через диалог — извлекаем, сохраняем в библиотеку,
/// делаем активным и применяем на оверлей.
pub fn load_from_user(app: &AppHandle, path: String) {
    if let Some(data) = loader::load_zip(&path) {
        let name = Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "crosshair".to_string());

        storage::add_crosshair(&name, &data.html, &data.css);

        sender::send(app, data);
        sender::notify_list_changed(app);
    }
}

/// Запуск приложения — подхватываем последний активный прицел из библиотеки.
pub fn restore_on_start(app: &AppHandle) {
    if let Some(id) = storage::get_active() {
        if let Some((html, css)) = storage::read_crosshair(&id) {
            sender::send(app, CrosshairData { html, css });
        }
    }
}

/// Клик по бару в настройках — применить прицел к оверлею.
pub fn apply_crosshair(app: &AppHandle, id: &str) {
    if let Some((html, css)) = storage::read_crosshair(id) {
        storage::set_active(id);
        sender::send(app, CrosshairData { html, css });
        sender::notify_list_changed(app);
    }
}

/// Удаление прицела из библиотеки (файлы + запись в settings.json).
pub fn delete_crosshair(app: &AppHandle, id: &str) {
    storage::delete_crosshair(id);
    sender::notify_list_changed(app);
}