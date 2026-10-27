pub mod init;
#[cfg(feature = "ely")]
pub mod login;
#[cfg(any(feature = "fabric", feature = "neoforge"))]
pub mod modloaders;
pub mod run;
pub mod test;
pub mod versions;

use clap::Subcommand;
#[cfg(feature = "ely")]
use login::LoginCommands;
#[cfg(any(feature = "fabric", feature = "neoforge"))]
use modloaders::ModloadersCommands;
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
    #[cfg(feature = "ely")]
    Login {
        #[command(subcommand)]
        subcommand: LoginCommands,
    },
    #[cfg(any(feature = "fabric", feature = "neoforge"))]
    Modloaders {
        #[command(subcommand)]
        subcommand: ModloadersCommands,
    },
}
