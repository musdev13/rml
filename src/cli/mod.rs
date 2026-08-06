use clap::Parser;

mod commands;
mod handler;

pub use handler::handle_cli;


#[derive(Parser)]
#[command(author, disable_version_flag = true, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<commands::Commands>,

    #[arg(short = 'V', long = "version", action = clap::ArgAction::SetTrue)]
    pub version: bool,
}


