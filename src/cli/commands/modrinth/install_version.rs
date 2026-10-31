use clap::Args;
use musutils::types::Status;
use std::path::PathBuf;

use rmlib::extra::modrinth::install;

use super::InstallFlags;

#[derive(Args, Debug)]
pub struct InstallVersionArgs {
    #[arg(help = "Version ID")]
    pub version_id: String,

    #[command(flatten)]
    pub flags: InstallFlags,
}

pub async fn handler(args: InstallVersionArgs, game_path: Option<PathBuf>, json: bool) {
    let params = match args.flags.to_params(game_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let result = match install::install_version(&args.version_id, &params).await {
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
        "{}: installed {} file(s)",
        Status::Ok.as_colored_str(),
        result.installed.len()
    );
    for f in &result.installed {
        println!("- {} ({} bytes)", f.path.display(), f.bytes);
    }
}
