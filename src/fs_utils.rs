use std::fs::File;
use std::io::{BufReader, Error, Read, Write};

pub fn read_file(path: &str) -> Result<File, Error> {
    File::open(path)
}

pub fn read_file_as_text(path: &str) -> Result<String, Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_file(path: String, content: String) -> Option<Error> {
    match File::create(path) {
        Ok(mut file) => file.write_all(content.as_bytes()).err(),
        Err(e) => Some(e),
    }
}
