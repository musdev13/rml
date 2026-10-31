use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::dto::{ProjectType, SearchIndex, SideSupport};
use rmlib::extra::modrinth::{SearchOptions, search};

#[derive(Args, Debug)]
pub struct SearchCliArgs {
    #[arg(help = "Search query")]
    pub query: Option<String>,

    #[arg(
        short = 't',
        long,
        value_name = "TYPE",
        help = "mod | modpack | resourcepack | shader | datapack | world"
    )]
    pub r#type: Option<String>,

    #[arg(
        long = "game-version",
        value_name = "VER",
        help = "Filter by Minecraft version"
    )]
    pub game_version: Vec<String>,

    #[arg(long, value_name = "LOADER", help = "Filter by mod loader")]
    pub loader: Vec<String>,

    #[arg(long, value_name = "CATEGORY", help = "Filter by category")]
    pub category: Vec<String>,

    #[arg(
        long,
        value_name = "SIDE",
        help = "client side: required | optional | unsupported | unknown"
    )]
    pub client_side: Option<String>,

    #[arg(
        long,
        value_name = "SIDE",
        help = "server side: required | optional | unsupported | unknown"
    )]
    pub server_side: Option<String>,

    #[arg(
        short = 'i',
        long,
        value_name = "INDEX",
        help = "relevance | downloads | follows | newest | updated"
    )]
    pub index: Option<String>,

    #[arg(short = 'p', long, default_value_t = 1, help = "Page number (1-based)")]
    pub page: u32,

    #[arg(
        short = 'c',
        long,
        default_value_t = 20,
        help = "Items per page (1..=100)"
    )]
    pub limit: u32,
}

fn parse_type(s: &str) -> Result<ProjectType, String> {
    match s {
        "mod" => Ok(ProjectType::Mod),
        "modpack" => Ok(ProjectType::Modpack),
        "resourcepack" => Ok(ProjectType::Resourcepack),
        "shader" => Ok(ProjectType::Shader),
        "datapack" => Ok(ProjectType::Datapack),
        "world" => Ok(ProjectType::World),
        other => Err(format!("invalid --type {other:?}")),
    }
}

fn parse_side(s: &str) -> Result<SideSupport, String> {
    match s {
        "required" => Ok(SideSupport::Required),
        "optional" => Ok(SideSupport::Optional),
        "unsupported" => Ok(SideSupport::Unsupported),
        "unknown" => Ok(SideSupport::Unknown),
        other => Err(format!("invalid side {other:?}")),
    }
}

fn parse_index(s: &str) -> Result<SearchIndex, String> {
    rmlib::extra::modrinth::search_index_from_str(s).ok_or_else(|| format!("invalid --index {s:?}"))
}

pub async fn handler(args: SearchCliArgs, json: bool) {
    let project_type = match args.r#type.as_deref().map(parse_type).transpose() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };
    let index = match args.index.as_deref().map(parse_index).transpose() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };
    let client_side = match args.client_side.as_deref().map(parse_side).transpose() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };
    let server_side = match args.server_side.as_deref().map(parse_side).transpose() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let opts = SearchOptions {
        query: args.query.clone(),
        project_type,
        game_versions: args.game_version,
        loaders: args.loader,
        categories: args.category,
        client_side,
        server_side,
        index,
        page: Some(args.page),
        limit: Some(args.limit),
        offset: None,
    };

    let response = match search(&opts).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: search failed: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&response) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    if response.hits.is_empty() {
        println!("no results.");
        return;
    }
    println!(
        "\n--- modrinth search: {} results (page {}, {} per page) ---",
        response.total_hits, args.page, response.limit
    );
    for hit in &response.hits {
        println!(
            "- {} ({}) [{}] ↓{} ♥{}",
            hit.title, hit.slug, hit.project_type, hit.downloads, hit.follows
        );
        if !hit.description.is_empty() {
            println!("    {}", hit.description);
        }
    }
    println!();
}
