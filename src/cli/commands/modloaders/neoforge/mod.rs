pub mod fetch_list;
pub mod install;

use clap::Subcommand;
use fetch_list::FetchListArgs;
use install::InstallArgs;

#[derive(Subcommand)]
pub enum NeoforgeCommands {
    #[command(name = "fetch-list")]
    FetchList(FetchListArgs),
    #[command()]
    Install(InstallArgs),
}
