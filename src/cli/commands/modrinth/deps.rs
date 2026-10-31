use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::project;

#[derive(Args, Debug)]
pub struct DepsArgs {
    #[arg(help = "Project ID or slug")]
    pub project_id: String,
}

pub async fn handler(args: DepsArgs, json: bool) {
    let project_id = match project::get_project(&args.project_id).await {
        Ok(p) => p.id,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let deps = match project::get_project_dependencies(&project_id).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&deps) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    if deps.is_empty() {
        println!("no dependencies.");
        return;
    }

    println!("\n--- {} dependency entr(y/ies) ---", deps.len());
    for d in &deps {
        println!(
            "- [{}] project: {}, version: {}",
            d.dependency_type,
            d.project_id.as_deref().unwrap_or("-"),
            d.version_id.as_deref().unwrap_or("-")
        );
    }
    println!();
}
