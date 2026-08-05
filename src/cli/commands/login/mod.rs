pub mod ely;

use clap::Subcommand;
use ely::ElyArgs;

#[derive(Subcommand)]
pub enum LoginCommands {
    #[command(name = "ely")]
    Ely(ElyArgs),
}
