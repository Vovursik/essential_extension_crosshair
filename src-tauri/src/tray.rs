use tauri::{
    Manager,
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};

use std::sync::{Arc, atomic::Ordering};

use crate::{state::AppState};
use crate::crosshair;

use tauri_plugin_dialog::DialogExt;

pub fn create_tray(app: &tauri::AppHandle) {
    let toggle_item =
        MenuItem::with_id(app, "toggle", "Mode: HOLD", true, None::<&str>).unwrap();

    let load_item =
        MenuItem::with_id(app, "load", "Load Crosshair (ZIP)", true, None::<&str>).unwrap();

    let quit_item =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    let icon = app.default_window_icon().unwrap().clone();

    let menu = Menu::with_items(app, &[&toggle_item, &load_item, &quit_item]).unwrap();

    let toggle_ref = Arc::new(toggle_item);
    let _app_handle = app.clone();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event({
            let toggle_ref = toggle_ref.clone();

            move |app, event| match event.id.as_ref() {

                "toggle" => {
                    let state = app.state::<Arc<AppState>>();

                    let new = !state.toggle_mode.load(Ordering::Relaxed);
                    state.toggle_mode.store(new, Ordering::Relaxed);

                    let text = if new { "Mode: TOGGLE" } else { "Mode: HOLD" };
                    toggle_ref.set_text(text).ok();

                    println!("mode -> {}", text);
                }

                "load" => {
                    let _app_handle = app.clone();

                    app.dialog()
                        .file()
                        .add_filter("zip", &["zip"])
                        .pick_file(move |file| {

                            if let Some(path) = file {

                                // ✅ FIX: FilePath → PathBuf → String
                                let path_buf = match path {
                                    tauri_plugin_dialog::FilePath::Path(p) => p,
                                    _ => return,
                                };

                                let path_str = path_buf.to_string_lossy().to_string();

                                crosshair::load_from_user(&_app_handle, path_str);
                            }
                        });
                }

                "quit" => {
                    std::process::exit(0);
                }

                _ => {}
            }
        })
        .build(app)
        .unwrap();
}