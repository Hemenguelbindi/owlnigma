use std::io;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::utils::print_owl;

// Определяем перечисление для команд
#[derive(Debug)]
enum Updates {
    NewConnection,
    Unknown,
}

impl Updates {
    // Преобразуем входящие данные в команду
    fn from_input(input: &str) -> Self {
        match input {
            "Knock knock!" => Updates::NewConnection,
            _ => Updates::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ServerOwl;

impl ServerOwl {
    pub fn new() -> ServerOwl {
        ServerOwl {}
    }

    // Запуск сервера
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

    // Обработка клиента
    async fn process_client(&self, mut socket: TcpStream) {
        let mut buffer = vec![0; 1024];
        let read_client = socket.read(&mut buffer).await.expect("Error reading data");

        if read_client == 0 {
            return;
        }

        // Преобразуем входящие данные в строку
        let received = String::from_utf8_lossy(&buffer[..read_client]).trim().to_string();

        // Определяем команду
        let command = Updates::from_input(&received);

        // Обрабатываем команду
        self.handle_client(command, &mut socket).await;
    }

    // Метод для обработки команды
    async fn handle_client(&self, command: Updates, socket: &mut TcpStream) {
        match command {
            Updates::NewConnection => {
                socket.write_all(b"Hello there! New connection established.\n").await.expect("Error writing data");
            }
            Updates::Unknown => {
                socket.write_all(b"Unknown command\n").await.expect("Error writing data");
            }
        }
    }
}
