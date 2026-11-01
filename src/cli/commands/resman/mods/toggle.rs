use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;

use rmlib::extra::resman::mods as rm;

use super::ModsCommonArgs;
use super::list::resolve_dir;

#[derive(Args, Debug, Clone)]
pub struct ToggleArgs {
    #[arg(help = "Mod id (matched against ModInfo::id)")]
    pub id: Option<String>,

    #[command(flatten)]
    pub common: ModsCommonArgs,
}

pub enum Action {
    Enable,
    Disable,
    Toggle,
}

pub fn handler(args: ToggleArgs, action: Action) {
    let target = match resolve_target(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let result = match action {
        Action::Enable => rm::enable(&target).map(|p| (p, true)),
        Action::Disable => rm::disable(&target).map(|p| (p, false)),
        Action::Toggle => rm::toggle(&target),
    };

    match result {
        Ok((path, enabled)) => {
            if args.common.json {
                println!(
                    "{}",
                    serde_json::json!({ "path": path.to_string_lossy(), "enabled": enabled })
                );
            } else {
                let state = if enabled { "enabled" } else { "disabled" };
                println!(
                    "{}: {} -> {} ({})",
                    Status::Ok.as_colored_str(),
                    target.display(),
                    path.display(),
                    state
                );
            }
        }
        Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
    }
}

fn resolve_target(args: &ToggleArgs) -> Result<PathBuf, String> {
    if let Some(f) = &args.common.file {
        return Ok(f.clone());
    }
    let dir = resolve_dir(&args.common)?;
    let id = args
        .id
        .as_deref()
        .ok_or_else(|| "mod id is required when --file is not given".to_string())?;
    let found = rm::info_by_id(&dir, id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("mod not found: {id}"))?;
    Ok(found.path)
}
