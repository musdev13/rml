pub mod test;
pub mod init;
pub mod versions;
pub mod run;

use clap::Subcommand;
use versions::VersionsCommands;
use run::RunArgs;

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
}
