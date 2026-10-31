use clap::Args;
use musutils::types::Status;
use std::path::PathBuf;

use rmlib::extra::modrinth::dto::{InstallResult, InstallSource, SkippedDependency};
use rmlib::extra::modrinth::install;

use super::InstallFlags;

#[derive(Args, Debug)]
pub struct InstallArgs {
    #[arg(
        value_name = "ID_OR_SLUG",
        num_args = 1..,
        help = "One or more project slugs/IDs (separate with spaces)"
    )]
    pub targets: Vec<String>,

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

    let mut merged = InstallResult {
        source: InstallSource::Project,
        project_type: None,
        installed: Vec::new(),
        skipped: Vec::<SkippedDependency>::new(),
    };

    let targets: Vec<String> = args
        .targets
        .iter()
        .flat_map(|t| t.split_whitespace().map(str::to_string))
        .collect();

    let mut failed = false;
    for target in &targets {
        match install::install_project(target, &params).await {
            Ok(result) => {
                if merged.project_type.is_none() {
                    merged.project_type = result.project_type;
                }
                merged.installed.extend(result.installed);
                merged.skipped.extend(result.skipped);
            }
            Err(e) => {
                failed = true;
                eprintln!(
                    "{}: install of {:?} failed: {}",
                    Status::Err.as_colored_str(),
                    target,
                    e
                );
            }
        }
    }

    if merged.installed.is_empty() && merged.skipped.is_empty() && failed {
        return;
    }

    super::output::print_result(&merged, json);
}
