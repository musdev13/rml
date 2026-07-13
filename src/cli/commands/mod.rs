pub mod test;
pub mod init;
pub mod versions;

use clap::Subcommand;
use versions::VersionsCommands;

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
}
