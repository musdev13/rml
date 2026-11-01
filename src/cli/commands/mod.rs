pub mod init;
pub mod login;
#[cfg(any(feature = "fabric", feature = "neoforge"))]
pub mod modloaders;
#[cfg(feature = "modrinth")]
pub mod modrinth;
pub mod resman;
pub mod run;
pub mod test;
pub mod versions;

use clap::Subcommand;
use login::LoginCommands;
#[cfg(any(feature = "fabric", feature = "neoforge"))]
use modloaders::ModloadersCommands;
#[cfg(feature = "modrinth")]
use modrinth::ModrinthArgs;
use resman::ResmanArgs;
use run::RunArgs;
use versions::VersionsCommands;

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand)]
pub enum Commands {
    Test,
    Init {
        #[arg(short, long)]
        force: bool,
    },
    Versions {
        #[command(subcommand)]
        subcommand: VersionsCommands,
    },
    Run(RunArgs),
    Resman(ResmanArgs),
    Login {
        #[command(subcommand)]
        subcommand: LoginCommands,
    },
    #[cfg(feature = "modrinth")]
    Modrinth(ModrinthArgs),
    #[cfg(any(feature = "fabric", feature = "neoforge"))]
    Modloaders {
        #[command(subcommand)]
        subcommand: ModloadersCommands,
    },
}
