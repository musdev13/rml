use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;

use rmlib::extra::resman::mods as rm;

use super::ModsCommonArgs;

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    #[arg(
        long,
        value_name = "FIELDS",
        help = "Comma-separated fields to show, or `all` (default: id,name,version,loader,minecraft,enabled)"
    )]
    pub fields: Option<String>,

    #[arg(
        long,
        value_name = "FORMAT",
        default_value = "table",
        help = "Output format: json | table | plain"
    )]
    pub format: String,

    #[command(flatten)]
    pub common: ModsCommonArgs,
}

pub async fn handler(args: ListArgs) {
    let dir = match resolve_dir(&args.common) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    const DEFAULT_FIELDS: &str = "id,name,version,loader,minecraft,enabled";

    let fields_spec = match args.fields.as_deref() {
        None => Some(DEFAULT_FIELDS.to_string()),
        Some("all") => None,
        Some(s) => Some(s.to_string()),
    };
    let fields = match fields_spec.as_deref().map(rm::parse_fields).transpose() {
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
        let list = match rm::scan_priority_dir(&dir, &priority, &fallback).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        if list.is_empty() {
            println!("no mods found.");
            return;
        }
        let value = match fields {
            Some(f) => serde_json::Value::Array(
                list.iter()
                    .map(|i| serde_json::Value::Object(rm::select_fields(i, &f)))
                    .collect(),
            ),
            None => serde_json::to_value(&list).unwrap_or_default(),
        };
        if !matches!(format, rm::Format::Json) {
            println!("\n--- mods list (items: {}) ---", list.len());
        }
        println!("{}", rm::render_value(&value, format));
        if !matches!(format, rm::Format::Json) {
            println!();
        }
        return;
    }

    match rm::scan_as(&dir, fields.as_deref(), format) {
        Ok(out) => {
            if out.trim().is_empty() {
                println!("no mods found.");
                return;
            }
            if !matches!(format, rm::Format::Json) {
                let count = rm::scan_dir(&dir).map(|l| l.len()).unwrap_or(0);
                println!("\n--- mods list (items: {count}) ---");
                println!("{out}");
                println!();
            } else {
                println!("{out}");
            }
        }
        Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
    }
}

pub(crate) fn resolve_dir(common: &ModsCommonArgs) -> Result<PathBuf, String> {
    if let Some(d) = &common.dir {
        return Ok(d.clone());
    }
    if let Some(p) = &common.file {
        return p
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| "file has no parent directory".to_string());
    }
    if let Some(g) = &common.game_path {
        return Ok(rm::mods_dir(g));
    }
    Err("either --game-path, --dir or --file is required".to_string())
}

pub(crate) fn parse_priority(common: &ModsCommonArgs) -> Result<rm::Priority, String> {
    match &common.priority {
        Some(p) => rm::Priority::parse(p).map_err(|e| e.to_string()),
        None => Ok(rm::Priority::default()),
    }
}

pub(crate) fn parse_fallback(common: &ModsCommonArgs) -> Result<rm::FallbackPolicy, String> {
    if common.skip {
        return Ok(rm::FallbackPolicy::Skip);
    }
    if let Some(f) = &common.fallback_fields {
        let fields = rm::parse_fields(f).map_err(|e| e.to_string())?;
        return Ok(rm::FallbackPolicy::Fields(fields));
    }
    Ok(rm::FallbackPolicy::default())
}
