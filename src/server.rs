use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::black_data::{decrypt_data, encrypt_data};
use crate::constant::SECRET_KEY;
use crate::utils::print_owl;

trait Command {
    async fn execute(&self, socket: &mut TcpStream);
}

enum Updates {
    NewConnection(NewConnection),
    Unknown,
}

impl Updates {
    pub async fn from_input(input: &[u8]) -> Self {
        let input_str = String::from_utf8_lossy(input);
        match input_str.trim() {
            "Knock knock!" => Updates::NewConnection(NewConnection),
            _ => Updates::Unknown,
        }
    }
}

struct NewConnection;

impl Command for NewConnection {
    async fn execute(&self, socket: &mut TcpStream) {
        let message = b"How there?\n";
        let crypto_data = encrypt_data(message, &SECRET_KEY);
        socket.write_u32(crypto_data.len() as u32).await.expect("Error writing length");
        socket.write_all(&crypto_data).await.expect("Error writing data");
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerOwl;

impl ServerOwl {
    pub fn new() -> ServerOwl {
        ServerOwl {}
    }

    pub async fn run_server(&self, address: &str) -> io::Result<()> {
        print_owl();
        println!("[ ] Server up...");
        let listener = TcpListener::bind(address).await.expect("Server error");
        println!("[ ] Server listening {}", address);

        let owl = Arc::new(*self);
        loop {
            let (socket, _) = listener.accept().await?;
            let owl_clone = Arc::clone(&owl);
            tokio::spawn(async move {
                owl_clone.process_client(socket).await;
            });
        }
    }

    async fn process_client(&self, mut socket: TcpStream) {
        let mut length_buffer = [0u8; 4];

        socket.read_exact(&mut length_buffer).await.expect("Error reading length");
        let length = u32::from_be_bytes(length_buffer) as usize;

        let mut buffer = vec![0u8; length];
        socket.read_exact(&mut buffer).await.expect("Error reading data");

        let decrypted_data = decrypt_data(&buffer, &get_secret_key());
        let command = Updates::from_input(&decrypted_data).await;
        println!("[ ] Client: {}", String::from_utf8_lossy(&decrypted_data));

        match command {
            Updates::NewConnection(command) => command.execute(&mut socket).await,
            Updates::Unknown => {
                let message = b"I don't understand you\n";
                let encrypted_message = encrypt_data(message, &get_secret_key());
                socket.write_u32(encrypted_message.len() as u32).await.expect("Error writing length");
                socket.write_all(&encrypted_message).await.expect("Error writing data");
            },
        }
    }
}
