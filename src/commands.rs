use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Owlnigma {
    #[command(subcommand)]
    pub subcommand: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Up{
        #[arg(default_value="127.0.0.1:5465")]
        address: String,
    },
    Connect{
        #[arg(default_value="127.0.0.1:5465")]
        address: String,
    },
}