use std::io::{stdout, Write};
use aead::Key;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, Nonce, OsRng},
    Aes256Gcm,
};

fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, Nonce<Aes256Gcm>) {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

    (ciphertext, nonce)
}

fn decrypt(key: &[u8; 32], nonce: &Nonce<Aes256Gcm>, ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    cipher.decrypt(nonce, ciphertext).unwrap()
}

/// Encrypt a string to a byte vector with nonce prepended
/// The output format is: [nonce (12 bytes) | ciphertext]
pub fn encrypt_string(key: Vec<u8>, plaintext: Vec<u8>) -> Result<Vec<u8>, String> {
    let key_obj = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key_obj);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit (12 bytes) nonce
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_slice())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt a byte vector (with prepended nonce) to a string
/// The input format is: [nonce (12 bytes) | ciphertext]
pub fn decrypt_string(key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, String> {
    if data.len() < 12 {
        return Err("Data too short: must contain at least 12-byte nonce".to_string());
    }

    // Extract nonce (first 12 bytes)
    let nonce = Nonce::<Aes256Gcm>::from_slice(&data[0..12]);

    // Extract ciphertext (remaining bytes)
    let ciphertext = &data[12..];

    let key_obj = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key_obj);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))
}
