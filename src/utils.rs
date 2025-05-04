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