use std::io;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::print_owl;

#[derive(Debug, Clone, Copy)]
pub struct ServerOwl;

impl ServerOwl {
    pub fn new() -> ServerOwl {
        ServerOwl{}
    }

    pub async fn run_server(&self, addres: &str)-> io::Result<()>{
        print_owl();
        println!("[ ] Server up...");
        let listener = TcpListener::bind(addres).await.expect("Server error");
        println!("[ ] Server listening {}", addres);
        loop {
            let (socket, _) = listener.accept().await?;
            self.process_client(socket).await;
        }
    }

    async fn process_client(self, mut socket: TcpStream){
        tokio::spawn( async move {
            let mut  buffer = vec![0; 1024];
    
            let read_client = socket.read(&mut buffer[..]).await.expect("Error read data");
            
            if read_client > 0 {
                if self.new_connection(&mut buffer, &mut socket).await {
                    socket.write_all(b"Hello, I'm a server").await.expect("Error write data");
                }
            }
        });
    }

    async fn new_connection(&self, buffer: &mut Vec<u8>, socket: &mut TcpStream) -> bool {
        let message = String::from_utf8_lossy(&mut buffer[..]);

        if message == "Knock knock!"{
            socket.write_all(b"Who there?").await.expect("Error write data");
        }
        return true;
    }
        

}
