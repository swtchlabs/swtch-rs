use std::io;
use std::fs::{File};
use std::io::{Read};

use ethers::types::H160;
use std::error::Error;

/// Convert an &str to an H160
pub fn str_to_h160(address: &str) -> Result<H160, Box<dyn Error>> {
    address.parse().map_err(|e| Box::new(e) as Box<dyn Error>)
}

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