use clap::Args;
use crate::core::{
    types::VersionType,
    version::{fetch_list, sort_by_type, strip_version},
};

#[derive(Args, Debug)]
pub struct FetchListArgs {
    #[arg(short, long, value_enum)]
    pub r#type: Option<VersionType>,

    #[arg(short, long, default_value_t = 0)]
    pub page: u32,

    #[arg(short, long, default_value_t = 10)]
    pub count: u32,

    #[arg(short, long)]
    pub json: bool,

    #[arg(long, help = "Include 'id' field in the output")]
    pub show_id: bool,

    #[arg(long, help = "Include 'releaseTime' field in the output")]
    pub show_release_time: bool,

    #[arg(long, help = "Include 'time' field in the output")]
    pub show_time: bool,

    #[arg(long, help = "Include 'type' field in the output")]
    pub show_type: bool,

    #[arg(long, help = "Include 'url' field in the output")]
    pub show_url: bool,
}

pub async fn handle(
    version_type: Option<VersionType>, // Теперь тут Option
    page: u32, 
    count: u32, 
    json: bool,
    id: bool,
    release_time: bool,
    time: bool,
    r#type: bool,
    url: bool,
) {
    let all_versions = fetch_list().await;
    let filtered_versions = match version_type {
        Some(t) => sort_by_type(&all_versions, t),
        None => all_versions,
    };
    let paged_versions = musutils::vec::get_page(&filtered_versions, page, count);
    let striped_versions = strip_version(
        &paged_versions, 
        id, 
        release_time, 
        time, 
        r#type, 
        url
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&striped_versions).expect("can't serialize"));
    } else {
        if striped_versions.is_empty() {
            println!("no versions available on page {}.", page);
            return;
        }
        println!("\n--- versions list (page {}, items: {}) ---", page, striped_versions.len());
        musutils::vec::print_json_as_table(&striped_versions);

        println!();
    }
}
