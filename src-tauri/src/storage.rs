use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CrosshairMeta {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
struct Settings {
    crosshairs: Vec<CrosshairMeta>,
    active: Option<String>,
    press_count_show: Option<usize>,
    press_count_hide: Option<usize>,
}

fn config_dir() -> PathBuf {
    dirs::config_local_dir().unwrap().join("visual-crosshair-v")
}

fn crosshairs_dir() -> PathBuf {
    config_dir().join("crosshairs")
}

fn settings_path() -> PathBuf {
    config_dir().join("settings.json")
}

fn load_settings() -> Settings {
    let Ok(contents) = fs::read_to_string(settings_path()) else {
        return Settings::default();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

fn save_settings(settings: &Settings) {
    fs::create_dir_all(config_dir()).ok();
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        fs::write(settings_path(), json).ok();
    }
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

fn now_millis() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

/// Сохраняет html/css на диск, добавляет запись в список и делает её активной.
pub fn add_crosshair(name: &str, html: &str, css: &str) -> CrosshairMeta {
    let id = format!("{}-{}", sanitize(name), now_millis());

    let dir = crosshairs_dir().join(&id);
    fs::create_dir_all(&dir).ok();
    fs::write(dir.join("index.html"), html).ok();
    fs::write(dir.join("style.css"), css).ok();

    let meta = CrosshairMeta { id: id.clone(), name: name.to_string() };

    let mut settings = load_settings();
    settings.crosshairs.push(meta.clone());
    settings.active = Some(id);
    save_settings(&settings);

    meta
}

pub fn list_crosshairs() -> Vec<CrosshairMeta> {
    load_settings().crosshairs
}

pub fn get_active() -> Option<String> {
    load_settings().active
}

pub fn set_active(id: &str) {
    let mut settings = load_settings();
    settings.active = Some(id.to_string());
    save_settings(&settings);
}

pub fn delete_crosshair(id: &str) {
    let mut settings = load_settings();
    settings.crosshairs.retain(|c| c.id != id);
    if settings.active.as_deref() == Some(id) {
        settings.active = None;
    }
    save_settings(&settings);

    fs::remove_dir_all(crosshairs_dir().join(id)).ok();
}

pub fn read_crosshair(id: &str) -> Option<(String, String)> {
    let dir = crosshairs_dir().join(id);
    let html = fs::read_to_string(dir.join("index.html")).ok()?;
    let css = fs::read_to_string(dir.join("style.css")).unwrap_or_default();
    Some((html, css))
}

pub fn get_press_count_show() -> usize {
    load_settings().press_count_show.unwrap_or(1).max(1)
}

pub fn save_press_count_show(count: usize) {
    let mut settings = load_settings();
    settings.press_count_show = Some(count.max(1));
    save_settings(&settings);
}

pub fn get_press_count_hide() -> usize {
    load_settings().press_count_hide.unwrap_or(1).max(1)
}

pub fn save_press_count_hide(count: usize) {
    let mut settings = load_settings();
    settings.press_count_hide = Some(count.max(1));
    save_settings(&settings);
}