#[cfg(feature = "fabric")]
pub mod fabric;
#[cfg(feature = "neoforge")]
pub mod neoforge;

use clap::Subcommand;
#[cfg(feature = "fabric")]
use fabric::FabricCommands;
#[cfg(feature = "neoforge")]
use neoforge::NeoforgeCommands;

#[derive(Subcommand)]
pub enum ModloadersCommands {
    #[cfg(feature = "fabric")]
    Fabric {
        #[command(subcommand)]
        subcommand: FabricCommands,
    },
    #[cfg(feature = "neoforge")]
    Neoforge {
        #[command(subcommand)]
        subcommand: NeoforgeCommands,
    },
}
