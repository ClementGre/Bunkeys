use aead::Key;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Nonce},
    Aes256Gcm
};

pub fn encrypt_test(key: &[u8; 32], plaintext: &[u8]) -> (Vec<u8>, Nonce<Aes256Gcm>) {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

    (ciphertext, nonce)
}

pub fn decrypt_test(key: &[u8; 32], nonce: &Nonce<Aes256Gcm>, ciphertext: &[u8]) -> Vec<u8> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    cipher.decrypt(nonce, ciphertext).unwrap()
}
