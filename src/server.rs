use std::io;
use std::sync::Arc;
use tokio::fs::File;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use crate::black_data::{decrypt_data, encrypt_data, get_secret_key};
use crate::utils::print_owl;


trait Command {
    async fn execute(&self, socket: &mut TcpStream);
}


enum Updates {
    NewConnection(NewConnection),
    Upload(Upload),
    Unknown,
}


impl Updates {
    pub async fn from_input(input: &[u8]) -> Self {
        let input_str = String::from_utf8_lossy(input);
        match input_str.trim() {
            "Knock knock!" => Updates::NewConnection(NewConnection),
            "Get this file" => Updates::Upload(Upload),
            _ => Updates::Unknown,
        }
    }
}


struct NewConnection;


impl Command for NewConnection {
    async fn execute(&self, socket: &mut TcpStream) {
        let message = b"How there?\n";
        let crypto_data = encrypt_data(message, &get_secret_key());
        socket.write_u32(crypto_data.len() as u32).await.expect("Error writing length");
        socket.write_all(&crypto_data).await.expect("Error writing data");
    }
}

struct Upload;

impl Command for Upload {
    async fn execute(&self, socket: &mut TcpStream) {
        let message = b"By quieter";
        println!("Original message: {:?}", message);
        
        let crypto_data = encrypt_data(message, &get_secret_key());
        socket.write_u32(crypto_data.len() as u32).await.expect("Error writing length");
        socket.write_all(&crypto_data).await.expect("Error writing data");

        let length = socket.read_u32().await.expect("Error reading length");
        let mut buffer = vec![0u8; length as usize];

        socket.read_exact(&mut buffer).await.expect("Error reading data");

        let mut descryp_data = decrypt_data(&buffer, &get_secret_key());


        while let Some(&0) = descryp_data.first() {
            descryp_data.remove(0);
        }

        while let Some(&0) = descryp_data.last() {
            descryp_data.pop();
        }
        
        let mut file = BufWriter::new(File::create("test.txt").await.unwrap());
        file.write_all(&descryp_data).await.unwrap();

        file.flush().await.unwrap();
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
            Updates::Upload(command) => command.execute(&mut socket).await,
            Updates::Unknown => {
                let message = b"I don't understand you\n";
                let encrypted_message = encrypt_data(message, &get_secret_key());
                socket.write_u32(encrypted_message.len() as u32).await.expect("Error writing length");
                socket.write_all(&encrypted_message).await.expect("Error writing data");
            },
        }
    }
}
