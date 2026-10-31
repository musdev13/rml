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

    super::output::print_result(&result, json);
}
