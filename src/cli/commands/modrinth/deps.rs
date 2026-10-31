use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::dto::{VersionFilters, VersionType};
use rmlib::extra::modrinth::{project, version};

#[derive(Args, Debug)]
pub struct DepsArgs {
    #[arg(help = "Project ID or slug")]
    pub project_id: String,

    #[arg(
        long = "version",
        value_name = "VERSION_ID",
        help = "Show dependencies of this specific version only"
    )]
    pub version_id: Option<String>,

    #[arg(long = "mc", value_name = "VER", help = "Filter by Minecraft version")]
    pub minecraft_version: Option<String>,

    #[arg(long, value_name = "LOADER", help = "Filter by mod loader")]
    pub loader: Option<String>,

    #[arg(
        long = "version-type",
        value_name = "TYPE",
        help = "release | beta | alpha"
    )]
    pub version_type: Option<String>,
}

fn parse_vt(s: &str) -> Result<VersionType, String> {
    match s {
        "release" => Ok(VersionType::Release),
        "beta" => Ok(VersionType::Beta),
        "alpha" => Ok(VersionType::Alpha),
        other => Err(format!("invalid --version-type {other:?}")),
    }
}

pub async fn handler(args: DepsArgs, json: bool) {
    let version_type = match args.version_type.as_deref().map(parse_vt).transpose() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let deps = if let Some(vid) = &args.version_id {
        match version::get_version(vid).await {
            Ok(v) => v
                .dependencies
                .into_iter()
                .map(|d| rmlib::extra::modrinth::dto::Dependency {
                    version_id: d.version_id,
                    project_id: d.project_id,
                    file_name: d.file_name,
                    dependency_type: d.dependency_type,
                })
                .collect::<Vec<_>>(),
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        }
    } else {
        let project_id = match project::get_project(&args.project_id).await {
            Ok(p) => p.id,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        let filters = VersionFilters {
            game_versions: args.minecraft_version.clone().map(|v| vec![v]),
            loaders: args.loader.clone().map(|l| vec![l]),
            version_type,
        };
        match project::get_project_dependencies_filtered(&project_id, &filters).await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
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
