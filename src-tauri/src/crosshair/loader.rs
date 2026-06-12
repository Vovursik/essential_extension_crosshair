use std::io::{Cursor, Read};
use zip::ZipArchive;

pub struct CrosshairData {
    pub html: String,
    pub css: String,
}

pub fn load_zip(path: &str) -> Option<CrosshairData> {
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