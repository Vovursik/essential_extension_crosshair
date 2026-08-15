use tauri::{
    tray::TrayIconBuilder,
    menu::{Menu, IconMenuItem},
    image::Image,
};
use tauri_plugin_dialog::DialogExt;

use crate::crosshair;

pub fn create_tray(app: &tauri::AppHandle) {
    let load_icon = Image::from_path("icons/tray/load.png").unwrap();
    let quit_icon = Image::from_path("icons/tray/quit.png").unwrap();

    let load_item = IconMenuItem::with_id(
        app, "load", "Load", true, Some(load_icon), None::<&str>,
    ).unwrap();

    let quit_item = IconMenuItem::with_id(
        app, "quit", "Quit", true, Some(quit_icon), None::<&str>,
    ).unwrap();

    let icon = Image::from_path("icons/32x32.png").unwrap();
    let menu = Menu::with_items(app, &[&load_item, &quit_item]).unwrap();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "load" => {
                let app_handle = app.clone();

                app.dialog()
                    .file()
                    .add_filter("", &["zip"])
                    .pick_file(move |file| {
                        if let Some(tauri_plugin_dialog::FilePath::Path(path)) = file {
                            crosshair::load_from_user(&app_handle, path.to_string_lossy().to_string());
                        }
                    });
            }
            "quit" => std::process::exit(0),
            _ => {}
        })
        .build(app)
        .unwrap();
}