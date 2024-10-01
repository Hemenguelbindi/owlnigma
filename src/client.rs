use tokio::fs;
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

    async fn check_connection(&self, mut stream: TcpStream){
        let message = b"Knock knock!";
        let encrypted_message = encrypt_data(message, &get_secret_key());

        stream.write_u32(encrypted_message.len() as u32).await.expect("Error client");
        stream.write_all(&encrypted_message).await.expect("Error client");
        println!("[>] Sent message: Knock knock!");

        let mut length_buffer = [0u8; 4];
        stream.read_exact(&mut length_buffer).await.expect("Error reading length");
        let length = u32::from_be_bytes(length_buffer) as usize;

        let mut buffer = vec![0u8; length];
        stream.read_exact(&mut buffer).await.expect("Error client");

        let decrypted_message = decrypt_data(&buffer, &get_secret_key());
        let response = String::from_utf8_lossy(&decrypted_message);

        println!("[>] Received message: {}", response);
    }

    pub async fn send_file(&self, filename: &str, address: &str) {
        print_owl();
        let mut stream = TcpStream::connect(address).await.expect("Error client");
        println!("[>] Connected to server...");
        let messeg = b"Get this file";
        let encrypted_messeg = encrypt_data(messeg, &get_secret_key());
        stream.write_u32(encrypted_messeg.len() as u32).await.expect("Error client");
        stream.write_all(&encrypted_messeg).await.expect("Error client");
        println!("[>] Sending file...");
        let mut file = fs::File::open(&filename).await.expect("Error client");
        let mut file_buffer = vec![0u8; 1024];
        file.read_to_end(&mut file_buffer).await.unwrap();
        let encrypted_file = encrypt_data(&file_buffer, &get_secret_key());
        stream.write_u32(encrypted_file.len() as u32).await.expect("Error client");
        stream.write_all(&encrypted_file).await.expect("Error client");
        println!("[>] File sent successfully!");
    }

 }


 #[cfg(test)]
 mod tests {
     use super::*;
     use tokio::net::TcpListener;
     use tokio::io::AsyncReadExt;
 
     #[tokio::test]
     async fn test_client_connect() {
         let address = "127.0.0.1:8080";
 
         let listener = TcpListener::bind(address).await.unwrap();
 
         tokio::spawn(async move {
             let client = ClientOwl::new();
             client.connect(address).await.unwrap();
         });
 
         let (mut socket, _) = listener.accept().await.unwrap();

         let mut length_buffer = [0u8; 4];
         socket.read_exact(&mut length_buffer).await.unwrap();
         let length = u32::from_be_bytes(length_buffer) as usize;
         let mut buffer = vec![0u8; length];
         socket.read_exact(&mut buffer).await.unwrap();
 

         let decrypted_message = decrypt_data(&buffer, &get_secret_key());
         let received_message = String::from_utf8_lossy(&decrypted_message);
 

         assert_eq!(received_message, "Knock knock!");
     }


     #[tokio::test]
    async fn test_check_connection() {
    let address = "127.0.0.1:8081";
    let listener = TcpListener::bind(address).await.unwrap();

    tokio::spawn(async move {
        let client = ClientOwl::new();
        client.connect(address).await.unwrap();
    });

    let (mut socket, _) = listener.accept().await.unwrap();


    let mut length_buffer = [0u8; 4];
    socket.read_exact(&mut length_buffer).await.unwrap();
    let length = u32::from_be_bytes(length_buffer) as usize;
    

    let mut buffer = vec![0u8; length];
    socket.read_exact(&mut buffer).await.unwrap();


    let decrypted_message = decrypt_data(&buffer, &get_secret_key());
    let received_message = String::from_utf8_lossy(&decrypted_message);
    assert_eq!(received_message, "Knock knock!");


    let response = b"Who's there?";
    let encrypted_response = encrypt_data(response, &get_secret_key());
    socket.write_u32(encrypted_response.len() as u32).await.unwrap();
    socket.write_all(&encrypted_response).await.unwrap();
    }
    
 }