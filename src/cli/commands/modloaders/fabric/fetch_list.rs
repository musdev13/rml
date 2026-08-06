use clap::Args;
use rmlib::core::modloaders::fabric::fetch_list;

#[derive(Args, Debug)]
pub struct FetchListArgs {
    pub version_id: String,

    #[arg(short, long, default_value_t = 0)]
    pub page: u32,

    #[arg(short, long, default_value_t = 10)]
    pub count: u32,

    #[arg(short, long)]
    pub json: bool,
}

pub async fn handler(args: FetchListArgs) {
    let all_versions = fetch_list(&args.version_id).await;
    let paged_versions = musutils::vec::get_page(&all_versions, args.page, args.count);

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&paged_versions).expect("can't serialize")
        );
    } else {
        if paged_versions.is_empty() {
            println!("no versions available on page {}.", args.page);
            return;
        }

        println!(
            "\n--- fabric loader versions for {} (page {}, items: {}) ---",
            args.version_id,
            args.page,
            paged_versions.len()
        );

        for version in &paged_versions {
            println!("- {}", version);
        }

        println!();
    }
}
