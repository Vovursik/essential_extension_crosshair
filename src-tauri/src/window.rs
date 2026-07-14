use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn open_settings_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("settings") {
        win.show().ok();
        win.set_focus().ok();
        return;
    }

    WebviewWindowBuilder::new(app, "settings", WebviewUrl::App("settings.html".into()))
        .title("Essential Extension Crosshair")
        .inner_size(640.0, 460.0)
        .min_inner_size(640.0, 460.0)
        .resizable(true)
        .center()
        .build()
        .unwrap();
}