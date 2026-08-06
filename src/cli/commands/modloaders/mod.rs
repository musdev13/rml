pub mod fabric;

use clap::Subcommand;
use fabric::FabricCommands;

#[derive(Subcommand)]
pub enum ModloadersCommands {
    Fabric {
        #[command(subcommand)]
        subcommand: FabricCommands,
    }
}
