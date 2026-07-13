use clap::{Parser, Subcommand};

pub mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Test,
}

pub fn handle_cli() -> bool {
    let args = Cli::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Test => {
                commands::test::handle();
            }
        }
        true
    } else {
        false
    }
}
