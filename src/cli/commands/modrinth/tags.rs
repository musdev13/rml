use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::tag;

#[derive(Args, Debug)]
pub struct TagsArgs {
    #[arg(
        help = "categories | loaders | game-versions | project-types",
        value_name = "KIND"
    )]
    pub kind: String,
}

pub async fn handler(args: TagsArgs, json: bool) {
    match args.kind.as_str() {
        "categories" => {
            let list = match tag::get_categories().await {
                Ok(l) => l,
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
            println!("\n--- {} categories ---", list.len());
            for c in &list {
                println!("- {} [{}]", c.name, c.project_type);
            }
            println!();
        }
        "loaders" => {
            let list = match tag::get_loaders().await {
                Ok(l) => l,
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
            println!("\n--- {} loaders ---", list.len());
            for l in &list {
                println!("- {} ({})", l.name, l.supported_project_types.join(", "));
            }
            println!();
        }
        "game-versions" => {
            let list = match tag::get_game_versions().await {
                Ok(l) => l,
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
            println!("\n--- {} game versions ---", list.len());
            for v in &list {
                println!("- {} ({}, major: {})", v.version, v.version_type, v.major);
            }
            println!();
        }
        "project-types" => {
            let list = match tag::get_project_types().await {
                Ok(l) => l,
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
            for t in &list {
                println!("- {t}");
            }
        }
        other => eprintln!(
            "{}: unknown tags kind {other:?} (expected categories, loaders, game-versions, project-types)",
            Status::Err.as_colored_str()
        ),
    }
}
