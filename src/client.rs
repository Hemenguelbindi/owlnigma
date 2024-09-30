use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::black_data::{encrypt_data, decrypt_data, get_secret_key};
use crate::utils::print_owl;

#[derive(Debug, Clone)]
pub struct ClientOwl;

impl ClientOwl {
    pub fn new() -> Self {
        ClientOwl {}
    }

    pub async fn connect(&self, address: &str) -> Result<(), std::io::Error> {
        print_owl();
        let stream = TcpStream::connect(address).await.expect("Error client");
        println!("[>] Connected to server...");
        self.check_connection(stream).await;
        Ok(())
    }

    async fn check_connection(&self, mut stream: TcpStream) {
        let message = b"Knock knock!";
        let encrypted_message = encrypt_data(message, &get_secret_key());

        // Отправляем длину зашифрованного сообщения
        stream.write_u32(encrypted_message.len() as u32).await.expect("Error client");
        stream.write_all(&encrypted_message).await.expect("Error client");
        println!("[>] Sent message: Knock knock!");

        // Читаем длину ответа
        let mut length_buffer = [0u8; 4];
        stream.read_exact(&mut length_buffer).await.expect("Error reading length");
        let length = u32::from_be_bytes(length_buffer) as usize;

        // Читаем зашифрованное сообщение
        let mut buffer = vec![0u8; length];
        stream.read_exact(&mut buffer).await.expect("Error client");

        let decrypted_message = decrypt_data(&buffer, &get_secret_key());
        let response = String::from_utf8_lossy(&decrypted_message);

        println!("[>] Received message: {}", response);
    }
}
