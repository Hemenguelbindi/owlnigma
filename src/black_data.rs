use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::Rng;

pub fn encrypt_data(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());

    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let nonce_slice = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce_slice, plaintext).expect("Ошибка шифрования");

    [nonce.as_slice(), &ciphertext].concat()
}

pub fn decrypt_data(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());

    let nonce = Nonce::from_slice(&ciphertext[..12]);
    let ciphertext = &ciphertext[12..];

    let plaintext = cipher.decrypt(nonce, ciphertext).expect("Ошибка дешифрования");

    plaintext
}

