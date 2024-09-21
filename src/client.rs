use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::print_owl;

#[derive(Debug,Clone)]
pub struct ClientOwl;

impl ClientOwl {
    pub fn new () -> Self {
        ClientOwl{}
    }


    pub async fn connect (&self, address: &str) -> Result<(), std::io::Error> {
        print_owl();
        let stream = TcpStream::connect(address).await.expect("Error client");
        println!("[>] Connect to server...");
        self.cheack_connect(stream).await;
        Ok(())
    }


    async fn cheack_connect(&self, mut stream: TcpStream){
        let message = "Knock knock!";
        stream.write_all(message.as_bytes()).await.expect("Error client");
        println!("[>] Sent message : {}", message);

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.expect("Error client");
        let response = String::from_utf8_lossy(&buffer);

        println!("[>] Received message : {}", response);
    }

}


