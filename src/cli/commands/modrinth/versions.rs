use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::dto::{VersionFilters, VersionType};
use rmlib::extra::modrinth::version;

#[derive(Args, Debug)]
pub struct VersionsArgs {
    #[arg(help = "Project ID or slug")]
    pub project_id: String,

    #[arg(long = "game-version", value_name = "VER")]
    pub game_version: Vec<String>,

    #[arg(long, value_name = "LOADER")]
    pub loader: Vec<String>,

    #[arg(long = "type", value_name = "TYPE", help = "release | beta | alpha")]
    pub version_type: Option<String>,
}

fn parse_vt(s: &str) -> Result<VersionType, String> {
    match s {
        "release" => Ok(VersionType::Release),
        "beta" => Ok(VersionType::Beta),
        "alpha" => Ok(VersionType::Alpha),
        other => Err(format!("invalid --type {other:?}")),
    }
}

pub async fn handler(args: VersionsArgs, json: bool) {
    let project_id = match rmlib::extra::modrinth::project::get_project(&args.project_id).await {
        Ok(p) => p.id,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let version_type = match args.version_type.as_deref().map(parse_vt).transpose() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let filters = VersionFilters {
        game_versions: if args.game_version.is_empty() {
            None
        } else {
            Some(args.game_version)
        },
        loaders: if args.loader.is_empty() {
            None
        } else {
            Some(args.loader)
        },
        version_type,
    };

    let list = match version::get_project_versions(&project_id, &filters).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&list) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    if list.is_empty() {
        println!("no versions matched.");
        return;
    }

    println!("\n--- {} version(s) ---", list.len());
    for v in &list {
        println!(
            "- {} [{}] {} — mc: {}, loaders: {}",
            v.version_number,
            v.version_type,
            v.id,
            v.game_versions.join(","),
            v.loaders.join(",")
        );
    }
    println!();
}
