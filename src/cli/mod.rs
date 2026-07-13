use clap::Parser;

mod commands;
mod handler;

pub use handler::handle_cli;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<commands::Commands>,
}


