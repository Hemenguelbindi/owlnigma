use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::print_owl;


trait Command {
    async fn execute(&self, socket: &mut TcpStream);
}


enum Updates {
    NewConnection(NewConnection),
    Unknown,
}

impl Updates {
    pub async fn from_input(input: &str) -> Self {
        match input {
            "Knock knock!" => Updates::NewConnection(NewConnection),
            _ => Updates::Unknown,
        }
    }

}

struct NewConnection;

impl Command for NewConnection {
    async fn execute(&self, socket: &mut TcpStream) {
        socket.write_all(b"How there?\n").await.expect("Error writing data");
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

        let mut buffer = vec![0; 1024];
        let read_client = socket.read(&mut buffer).await.expect("Error reading data");

        if read_client == 0 {
            return;
        }

        let received = String::from_utf8_lossy(&buffer[..read_client]).trim().to_string();
        println!("[ ] Client: {}", received);
        
        let command = Updates::from_input(&received).await;
        
        match command {
            Updates::NewConnection(command) => command.execute(&mut socket).await,
            Updates::Unknown => socket.write_all(b"I don't understand you\n").await.expect("Error writing data"),
        }
    }
}


























