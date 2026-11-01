use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;

use rmlib::extra::resman::mods as rm;

use super::ModsCommonArgs;
use super::list::{parse_fallback, parse_priority, resolve_dir};

#[derive(Args, Debug, Clone)]
pub struct InfoArgs {
    #[arg(help = "Mod id (matched against ModInfo::id)")]
    pub id: Option<String>,

    #[arg(
        long,
        value_name = "FIELDS",
        help = "Comma-separated fields to show (default: all)"
    )]
    pub fields: Option<String>,

    #[arg(
        long,
        value_name = "FORMAT",
        default_value = "plain",
        help = "Output format: json | table | plain"
    )]
    pub format: String,

    #[command(flatten)]
    pub common: ModsCommonArgs,
}

pub async fn handler(args: InfoArgs) {
    let fields = match args.fields.as_deref().map(rm::parse_fields).transpose() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let format = if args.common.json {
        rm::Format::Json
    } else {
        match rm::Format::parse(&args.format) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        }
    };

    let target_path = match resolve_target(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let uses_priority =
        args.common.priority.is_some() || args.common.skip || args.common.fallback_fields.is_some();

    if uses_priority {
        let priority = match parse_priority(&args.common) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        let fallback = match parse_fallback(&args.common) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        let info = match rm::info_priority(&target_path, &priority, &fallback).await {
            Ok(Some(i)) => i,
            Ok(None) => {
                eprintln!(
                    "{}: skipped by fallback policy",
                    Status::Warn.as_colored_str()
                );
                return;
            }
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        let value = match fields {
            Some(f) => serde_json::Value::Object(rm::select_fields(&info, &f)),
            None => serde_json::to_value(&info).unwrap_or_default(),
        };
        println!("{}", rm::render_value(&value, format));
        return;
    }

    match rm::info_file_as(&target_path, fields.as_deref(), format) {
        Ok(out) => println!("{out}"),
        Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
    }
}

fn resolve_target(args: &InfoArgs) -> Result<PathBuf, String> {
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
