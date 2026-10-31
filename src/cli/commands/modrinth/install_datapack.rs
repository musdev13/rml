use clap::Args;
use musutils::types::Status;
use std::path::PathBuf;

use rmlib::extra::modrinth::install;

use super::InstallFlags;

#[derive(Args, Debug)]
pub struct InstallDatapackArgs {
    #[arg(help = "Datapack slug or ID")]
    pub id_or_slug: String,

    #[arg(
        short = 'w',
        long,
        value_name = "WORLD",
        help = "World name (folder under saves/)"
    )]
    pub world: String,

    #[command(flatten)]
    pub flags: InstallFlags,
}

pub async fn handler(args: InstallDatapackArgs, game_path: Option<PathBuf>, json: bool) {
    let Some(game_path) = game_path else {
        eprintln!(
            "{}: --game-path is required for install-datapack",
            Status::Err.as_colored_str()
        );
        return;
    };

    let params = match args.flags.to_params(Some(game_path.clone())) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let result =
        match install::install_datapack(&args.id_or_slug, &args.world, &game_path, &params).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{}: install failed: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };

    if json {
        match serde_json::to_string_pretty(&result) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    println!(
        "{}: installed {} datapack file(s) into world {:?}",
        Status::Ok.as_colored_str(),
        result.installed.len(),
        args.world
    );
    for f in &result.installed {
        println!("- {} ({} bytes)", f.path.display(), f.bytes);
    }
}
