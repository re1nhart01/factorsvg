use std::path::{Path};

pub fn is_input_not_file(content: String) -> bool {
   match content.split('.').nth(1) {
        Some(part) => !part.is_empty(),
        None => true,
    }
}

#[warn(unused)]
pub fn remove_filename(path_str: &str) -> String {
    let path = Path::new(path_str);
    match path.parent() {
        Some(parent) => parent.to_string_lossy().to_string(),
        None => String::new(),
    }
}

pub fn extract_viewbox_width(viewbox: String) -> u32 {
    let parts: Vec<&str> = viewbox.trim().split_whitespace().collect();

    if parts.len() == 4 {
        if let Ok(width) = parts[2].parse::<u32>() {
            return width;
        }
    }

    0u32
}

pub fn extract_viewbox_height(viewbox: String) -> u32 {
    let parts: Vec<&str> = viewbox.trim().split_whitespace().collect();

    if parts.len() == 4 {
        if let Ok(width) = parts[3].parse::<u32>() {
            return width;
        }
    }

    0u32
}