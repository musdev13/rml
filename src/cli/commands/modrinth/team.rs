use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::user;

#[derive(Args, Debug)]
pub struct TeamArgs {
    #[arg(help = "Team ID")]
    pub team_id: String,
}

pub async fn handler(args: TeamArgs, json: bool) {
    let members = match user::get_team_members(&args.team_id).await {
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
