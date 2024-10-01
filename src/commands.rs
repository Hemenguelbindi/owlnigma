use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(author="Hemenguelbindi", version="0.0.1", about="CLI AES server", long_about = None)]
pub struct Owlnigma {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start up server deflaut 127.0.0.1:5465
    Up{
        #[arg(default_value="127.0.0.1:5465", short, long)]
        address: String,
    },
    /// Connect to server deflaut 127.0.0.1:5465
    Connect{
        #[arg(default_value="127.0.0.1:5465")]
        address: String,
    },
    /// Send file to server deflaut 127.0.0.1:5465
    SendFile{
        /// ipaddres and port deflaut 127.0.0.1:5465
        #[arg(default_value="127.0.0.1:5465")]
        address: String,
        /// file name deflaut test.txt
        #[arg(default_value="test.txt")]
        filename: String,
    },
}