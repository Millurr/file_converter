use std::{fs::File, error::Error, io::Write};


pub fn write_to_file(path: String, input: String) -> Result<(), Box<dyn Error>> {
    println!("Writing to file...");
    let mut file = File::create(path)?;
    _ = file.write_all(input.as_bytes())?;
    Ok(())
}