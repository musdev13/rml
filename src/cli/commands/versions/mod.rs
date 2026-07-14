pub mod fetch_list;
pub mod install;

use clap::Subcommand;
use fetch_list::FetchListArgs;
use install::InstallArgs;


#[derive(Subcommand)]
pub enum VersionsCommands {
    #[command(name = "fetch-list")]
    FetchList(FetchListArgs),

    #[command(name = "install")]
    Install(InstallArgs)
}
