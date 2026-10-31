use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::user;

#[derive(Args, Debug)]
pub struct UserProjectsArgs {
    #[arg(help = "User ID")]
    pub user_id: String,
}

pub async fn handler(args: UserProjectsArgs, json: bool) {
    let projects = match user::get_user_projects(&args.user_id).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&projects) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    if projects.is_empty() {
        println!("no projects.");
        return;
    }

    println!("\n--- {} project(s) ---", projects.len());
    for p in &projects {
        println!(
            "- {} ({}) [{}] ↓{}",
            p.title, p.slug, p.project_type, p.downloads
        );
    }
    println!();
}
