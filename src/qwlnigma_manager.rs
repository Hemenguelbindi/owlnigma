use crate::commands::Commands;
use crate::server::ServerOwl;
use crate::client::ClientOwl;




pub struct QwlnigmaManager;



impl QwlnigmaManager {
    pub fn new() -> QwlnigmaManager {
        QwlnigmaManager{}
    }

    pub async fn execute(&self, commnads:Commands ) {
        match commnads {
            Commands::Up{address} =>{
                let server = ServerOwl::new();
                server.run_server(&address).await.expect("Error commands");
            }
            Commands::Connect { address } =>{
                let client = ClientOwl::new();
                client.connect(&address).await.expect("Error commands");
            }
       }
    }
}