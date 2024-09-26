use std::env;
use base64::decode;

pub fn get_secret_key() -> [u8; 32] {
    let key_base64 = env::var("SECRET_KEY").expect("SECRET_KEY is not set in the environment");
    let key_bytes = decode(&key_base64).expect("Failed to decode base64 key");

    if key_bytes.len() != 32 {
        panic!("Decoded key is not 32 bytes");
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&key_bytes);
    key
}
