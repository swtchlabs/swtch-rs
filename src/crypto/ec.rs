use ecies::{
    decrypt, 
    encrypt, 
    utils::generate_keypair,
    PublicKey,
    SecretKey
};

use std::fs::{File};
use std::io::{Read, Write};

use crate::utils::read_hex_from_file;

/// Generate New PrivateKey and PublicKey
pub fn new_keypair() -> (SecretKey, PublicKey) {
    generate_keypair()
}

/// Encrypt File with Public Key
pub fn encrypt_file(file_path: &str, public_key_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Encrypting file: {}", file_path);
    
    // Read the binary file
    let mut file = File::open(file_path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?; 
    
    // Read the hex string from the file
    let hex_string = read_hex_from_file(public_key_path)?;
    println!("Public key read from file: {}", hex_string);
    let public_key_bytes = hex::decode(&hex_string).expect("Failed to decode public key");

    // Encrypt the data
    let encrypted_data = encrypt(&public_key_bytes, &file_data).unwrap();

    // Write the encrypted data to a file
    let mut output = File::create(output_path)?;
    output.write_all(&encrypted_data)?;

    Ok(())
}

/// Decrypt File with Private Key
pub fn decrypt_file(file_path: &str, secret_key_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Decrypting file: {}", file_path);

    // Read the binary file
    let mut file = File::open(file_path)?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)?;

    // Read the hex string from the file
    let hex_string = read_hex_from_file(secret_key_path)?;
    println!("Secret key read from file: {}", hex_string);
    let secret_key_bytes = hex::decode(&hex_string).expect("Failed to decode secret key");

    // Decrypt the data
    let decrypted_data = decrypt(&secret_key_bytes, &file_data).unwrap();

    // Write the decrypted data to a file
    let mut decrypted_output = File::create(output_path)?;
    decrypted_output.write_all(&decrypted_data)?;

    Ok(())
}

