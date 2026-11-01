#[cfg(feature = "resman-mods")]
pub mod mods;
#[cfg(feature = "resman-resourcepacks")]
pub mod resourcepacks;

use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct ResmanArgs {
    #[command(subcommand)]
    pub subcommand: ResmanCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ResmanCommands {
    #[cfg(feature = "resman-mods")]
    Mods(mods::ModsArgs),
    #[cfg(feature = "resman-resourcepacks")]
    Resourcepacks(resourcepacks::ResourcepacksArgs),
}
