#[cfg(feature = "ely")]
pub mod ely;
pub mod ms;

use clap::Subcommand;
#[cfg(feature = "ely")]
use ely::ElyArgs;
use ms::MsArgs;

#[derive(Subcommand, Debug)]
pub enum LoginCommands {
    #[cfg(feature = "ely")]
    #[command(name = "ely")]
    Ely(ElyArgs),
    #[command(name = "ms")]
    Ms(MsArgs),
}
