use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::version;

#[derive(Args, Debug)]
pub struct VersionByHashArgs {
    #[arg(help = "SHA-1 or SHA-512 hash")]
    pub hash: String,
}

pub async fn handler(args: VersionByHashArgs, json: bool) {
    let v = match version::get_version_by_hash(&args.hash).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&v) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    println!(
        "{} {} ({}) — version id {}, project {}",
        Status::Ok.as_colored_str(),
        v.name,
        v.version_number,
        v.id,
        v.project_id
    );
}
