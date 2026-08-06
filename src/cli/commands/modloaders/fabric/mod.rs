pub mod fetch_list;

use clap::Subcommand;
use fetch_list::FetchListArgs;

#[derive(Subcommand)]
pub enum FabricCommands {
    #[command(name = "fetch-list")]
    FetchList(FetchListArgs),
}
