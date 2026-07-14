use tauri::{
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};

use crate::window::open_settings_window;

pub fn create_tray(app: &tauri::AppHandle) {
    let open_item =
        MenuItem::with_id(app, "open", "Preferences", true, None::<&str>).unwrap();

    let quit_item =
        MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    let icon = app.default_window_icon().unwrap().clone();

    let menu = Menu::with_items(app, &[&open_item, &quit_item]).unwrap();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                open_settings_window(app);
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .build(app)
        .unwrap();
}