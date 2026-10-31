use clap::Args;
use musutils::types::Status;
use std::path::PathBuf;

use rmlib::extra::modrinth::install;

use super::InstallFlags;

#[derive(Args, Debug)]
pub struct InstallArgs {
    #[arg(help = "Project slug or ID")]
    pub id_or_slug: String,

    #[command(flatten)]
    pub flags: InstallFlags,
}

pub async fn handler(args: InstallArgs, game_path: Option<PathBuf>, json: bool) {
    let params = match args.flags.to_params(game_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let result = match install::install_project(&args.id_or_slug, &params).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}: install failed: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    super::output::print_result(&result, json);
}
