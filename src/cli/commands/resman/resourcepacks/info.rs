use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;

use rmlib::extra::resman::resourcepacks as rp;

use super::RpCommonArgs;
use super::list::{parse_fallback, parse_priority, resolve_dir};

#[derive(Args, Debug, Clone)]
pub struct InfoArgs {
    #[arg(
        value_name = "NAME",
        help = "Resource pack file name without .zip / .disabled (for example \"Faithful 32x - 26.3\"); ignored with --file"
    )]
    pub id: Option<String>,

    #[arg(
        long,
        value_name = "FIELDS",
        help = "Comma-separated fields (default: all)"
    )]
    pub fields: Option<String>,

    #[arg(
        long,
        value_name = "FORMAT",
        default_value = "plain",
        help = "json | table | plain"
    )]
    pub format: String,

    #[command(flatten)]
    pub common: RpCommonArgs,
}

pub async fn handler(args: InfoArgs) {
    let fields = match args.fields.as_deref().map(rp::parse_fields).transpose() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    let format = if args.common.json {
        rp::Format::Json
    } else {
        match rp::Format::parse(&args.format) {
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
        let info = match rp::info_priority(&target_path, &priority, &fallback).await {
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
            Some(f) => serde_json::Value::Object(rp::select_fields(&info, &f)),
            None => serde_json::to_value(&info).unwrap_or_default(),
        };
        println!("{}", rp::render_value(&value, format));
        return;
    }

    match rp::info_file_as(&target_path, fields.as_deref(), format) {
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
        .ok_or_else(|| "resource pack name is required when --file is not given".to_string())?;
    let list = rp::scan_dir(&dir).map_err(|e| e.to_string())?;
    let found = list
        .into_iter()
        .find(|p| p.filename == id)
        .ok_or_else(|| format!("resource pack not found: {id}"))?;
    Ok(found.path)
}
