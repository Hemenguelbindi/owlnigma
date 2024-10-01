use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use rand::Rng;
use std::env;
use base64::{engine::general_purpose, Engine};


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


pub fn get_secret_key() -> [u8; 32] {
    // Получаем переменную окружения
    let secret_key_base64 = env::var("SECRET_KEY").expect("SECRET_KEY is not set");

    // Используем новый API для декодирования
    let decoded_key = general_purpose::STANDARD
        .decode(&secret_key_base64)
        .expect("Failed to decode SECRET_KEY");

    // Проверяем, что ключ правильной длины
    decoded_key
        .try_into()
        .expect("Invalid key length, expected 32 bytes")
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_encrypt_and_desrypt() {
        let key = get_secret_key();
        let data = b"Hello world!";
        let encrypted = encrypt_data(data, &key);
        assert_ne!(data, &encrypted[..]);

        let decrypted = decrypt_data(&encrypted, &key);
        assert_eq!(data, &decrypted[..]);
    }

    #[test]
    fn test_get_secret_key() {
        let key = get_secret_key();
        assert_eq!(key.len(), 32);
    }
}