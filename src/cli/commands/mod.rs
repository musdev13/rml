pub mod test;
pub mod init;
pub mod versions;
pub mod run;
pub mod login;
pub mod modloaders;

use clap::Subcommand;
use versions::VersionsCommands;
use run::RunArgs;
use login::LoginCommands;
use modloaders::ModloadersCommands;

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
    }
}
