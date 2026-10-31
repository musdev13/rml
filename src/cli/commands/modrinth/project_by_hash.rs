use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::project;

#[derive(Args, Debug)]
pub struct ProjectByHashArgs {
    #[arg(help = "SHA-1 or SHA-512 hash of a file")]
    pub hash: String,
}

pub async fn handler(args: ProjectByHashArgs, json: bool) {
    let p = match project::get_project_by_hash(&args.hash).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&p) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    println!(
        "{} {} ({}) — id: {}, type: {}, downloads: {}",
        Status::Ok.as_colored_str(),
        p.title,
        p.slug,
        p.id,
        p.project_type,
        p.downloads
    );
}
