pub mod init;
pub mod login;
pub mod modloaders;
pub mod run;
pub mod test;
pub mod versions;

use clap::Subcommand;
use login::LoginCommands;
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
    Login {
        #[command(subcommand)]
        subcommand: LoginCommands,
    },
    Modloaders {
        #[command(subcommand)]
        subcommand: ModloadersCommands,
    },
}
