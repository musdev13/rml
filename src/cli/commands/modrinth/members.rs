use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::project;

#[derive(Args, Debug)]
pub struct MembersArgs {
    #[arg(help = "Project ID or slug")]
    pub project_id: String,
}

pub async fn handler(args: MembersArgs, json: bool) {
    let project_id = match project::get_project(&args.project_id).await {
        Ok(p) => p.id,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let members = match project::get_project_members(&project_id).await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&members) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    if members.is_empty() {
        println!("no members.");
        return;
    }

    println!("\n--- {} member(s) ---", members.len());
    for m in &members {
        let name = m
            .user
            .as_ref()
            .map(|u| u.name.clone().unwrap_or_else(|| u.username.clone()))
            .unwrap_or_else(|| "-".to_string());
        println!("- {} — role: {}", name, m.role);
    }
    println!();
}
