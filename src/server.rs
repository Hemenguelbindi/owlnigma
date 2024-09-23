use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::print_owl;

#[derive(Debug, Clone, Copy)]
pub struct ServerOwl;

impl ServerOwl {
    pub fn new() -> ServerOwl {
        ServerOwl{}
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
        self.inspect(ServerHandlers::NewConnections, &mut buffer, &mut socket).await;
    }

    async fn inspect(&self, event: ServerHandlers, buffer: &mut Vec<u8>, socket: &mut TcpStream) {
        match event {
            ServerHandlers::NewConnections => {
                NewConnections::new_connection(buffer, socket).await;
            },
        }
    }
}

enum ServerHandlers {
    NewConnections,
}

struct NewConnections;

impl NewConnections {
    async fn new_connection(buffer: &mut Vec<u8>, socket: &mut TcpStream) {
        println!("[ ] New connection!");
        let message = String::from_utf8_lossy(&buffer[..]);
        println!("[ ] Message received: {}", message);

        if message.trim() == "Knock knock!" {
            println!("[ ] Sending response: Who's there?");
            socket.write_all(b"Who's there?").await.expect("Error writing data");
            println!("[ ] Response sent!");
        }
    }
}

