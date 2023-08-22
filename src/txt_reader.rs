use std::{fs, error::Error};


pub fn read_file_from_path(path: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    Ok(contents)
}