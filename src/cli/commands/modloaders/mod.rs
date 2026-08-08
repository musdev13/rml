pub mod fabric;
pub mod neoforge;

use clap::Subcommand;
use fabric::FabricCommands;
use neoforge::NeoforgeCommands;

#[derive(Subcommand)]
pub enum ModloadersCommands {
    Fabric {
        #[command(subcommand)]
        subcommand: FabricCommands,
    },
    Neoforge {
        #[command(subcommand)]
        subcommand: NeoforgeCommands,
    }
}
