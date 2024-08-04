use std::io;
use std::fs::{File};
use std::io::{Read};

/// Save a public or secret key to file
pub fn save_key_to_file(key_data: &str, file_path: &str) -> Result<(), std::io::Error> {
    std::fs::write(file_path, key_data)?;
    Ok(())
}

/// Reads a file and returns its contents as a hex string.
pub fn read_hex_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hex_string = String::new();
    file.read_to_string(&mut hex_string)?;
    Ok(hex_string)
}