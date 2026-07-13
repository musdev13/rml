pub mod fetch_list;

use clap::Subcommand;
use crate::core::types::VersionType;



#[derive(Subcommand)]
pub enum VersionsCommands {
    #[command(name = "fetch-list")]
    FetchList {
        #[arg(short, long, value_enum, default_value_t = VersionType::Release)]
        r#type: VersionType,

        #[arg(short, long, default_value_t = 0)]
        page: u32,

        #[arg(short, long, default_value_t = 10)]
        count: u32,

        #[arg(short, long)]
        json: bool,

        #[arg(long, help = "Include 'id' field in the output")]
        show_id: bool,

        #[arg(long, help = "Include 'releaseTime' field in the output")]
        show_release_time: bool,

        #[arg(long, help = "Include 'time' field in the output")]
        show_time: bool,

        #[arg(long, help = "Include 'type' field in the output")]
        show_type: bool,

        #[arg(long, help = "Include 'url' field in the output")]
        show_url: bool,
    }
}
