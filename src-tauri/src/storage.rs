use std::fs;
use dirs;

pub fn save_path(path: &str) {
    let config_dir = dirs::config_local_dir()
        .unwrap()
        .join("visual-crosshair-v");
    
    std::fs::create_dir_all(&config_dir).ok();
    
    let config_file = config_dir.join("settings.json");
    let settings = serde_json::json!({
        "last_crosshair": path
    });
    
    std::fs::write(config_file, serde_json::to_string_pretty(&settings).unwrap()).ok();
}

pub fn load_path() -> Option<String> {
    let config_dir = dirs::config_local_dir()
        .unwrap()
        .join("visual-crosshair-v");
    
    let config_file = config_dir.join("settings.json");
    
    // Читаем файл
    let contents = fs::read_to_string(config_file).ok()?;
    
    // Парсим JSON
    let settings: serde_json::Value = serde_json::from_str(&contents).ok()?;
    
    // Извлекаем путь
    settings.get("last_crosshair")?
        .as_str()
        .map(String::from)
}