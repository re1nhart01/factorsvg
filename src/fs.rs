use std::fs::File;
use std::io::{BufReader, Error, Read, Write};

pub fn read_file(path: &str) -> Result<File, Error> {
    let file = File::open(path);
    match file {
        Ok(value) => Ok(value),
        Err(error) => Err(error),
    }
}

pub fn read_file_as_text(path: &str) -> Result<String, Error> {
    let file = File::open(path)?; 
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_file(path: String, content: String) -> Option<Error> {
    let new_file = File::create(path);

    match new_file {
        Ok(mut file_descr) => {
            if let Err(err) = file_descr.write_all(content.as_bytes()) {
                return Some(err)
            }
            return None
        }
        Err(error) => {
            println!("{}", error)
        }
    }

    return None
}