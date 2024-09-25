mod utils;
mod commands;
mod qwlnigma_manager;
mod server;
mod client;
mod black_data;
mod constant;
use clap::Parser;
use qwlnigma_manager::QwlnigmaManager;

use crate::commands::Owlnigma;


#[tokio::main]
async fn main() {
    let args = Owlnigma::parse();
    let qwlnigma_manager = QwlnigmaManager::new();

    qwlnigma_manager.execute(args.subcommand).await;
}