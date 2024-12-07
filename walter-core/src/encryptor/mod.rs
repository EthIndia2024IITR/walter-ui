use openssl::pkcs5::pbkdf2_hmac;
use openssl::symm::{decrypt, encrypt, Cipher};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};

const SALT_LEN: usize = 16; // Salt length in bytes
const IV_LEN: usize = 16; // AES-256-CBC IV length
const KEY_LEN: usize = 32; // AES-256 requires a 256-bit (32 bytes) key
const PBKDF2_ITERATIONS: usize = 10000;

pub fn encrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read input file contents
    let mut file = File::open(input_file)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;

    // Generate a random salt
    let mut salt = [0u8; SALT_LEN];
    rand::thread_rng().fill(&mut salt);

    // Derive a key from the password using PBKDF2
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac(
        password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        openssl::hash::MessageDigest::sha256(),
        &mut key,
    )?;

    // Generate a random IV
    let mut iv = [0u8; IV_LEN];
    rand::thread_rng().fill(&mut iv);

    // Encrypt the data using AES-256-CBC
    let ciphertext = encrypt(Cipher::aes_256_cbc(), &key, Some(&iv), &plaintext)?;

    // Prepare the output
    let mut output = vec![];
    output.extend_from_slice(&salt); // Include salt at the beginning
    output.extend_from_slice(&iv); // Include IV after the salt
    output.extend_from_slice(&ciphertext); // Append the ciphertext

    // Write to the output file
    let mut out_file = File::create(output_file)?;
    out_file.write_all(&output)?;

    println!("File encrypted successfully.");
    Ok(())
}

pub fn decrypt_file(
    input_file: &str,
    output_file: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(input_file)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let salt = &data[0..SALT_LEN];
    let iv = &data[SALT_LEN..SALT_LEN + IV_LEN];
    let ciphertext = &data[SALT_LEN + IV_LEN..];

    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac(
        password.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        openssl::hash::MessageDigest::sha256(),
        &mut key,
    )?;

    let plaintext = decrypt(Cipher::aes_256_cbc(), &key, Some(iv), ciphertext)?;

    let mut out_file = File::create(output_file)?;
    out_file.write_all(&plaintext)?;

    println!("File decrypted successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_encrypt_decrypt_file() {
        let input_file = "tests/test.txt";
        let encrypted_file = "tests/test.txt.enc";
        let decrypted_file = "tests/test.txt.dec";
        let password = "password";

        // Create a test file
        let mut file = File::create(input_file).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        // Encrypt the file
        encrypt_file(input_file, encrypted_file, password).unwrap();

        // Decrypt the file
        decrypt_file(encrypted_file, decrypted_file, password).unwrap();

        // Read the decrypted file
        let mut decrypted_content = String::new();
        let mut file = File::open(decrypted_file).unwrap();
        file.read_to_string(&mut decrypted_content).unwrap();

        // Clean up
        fs::remove_file(input_file).unwrap();
        fs::remove_file(encrypted_file).unwrap();
        fs::remove_file(decrypted_file).unwrap();

        assert_eq!(decrypted_content, "Hello, World!");
    }
}
