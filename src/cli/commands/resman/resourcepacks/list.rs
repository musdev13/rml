use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;

use rmlib::extra::resman::resourcepacks as rp;

use super::RpCommonArgs;

#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    #[arg(
        long,
        value_name = "FIELDS",
        help = "Comma-separated fields, or `all` (default: filename,pack_format,description; `id` is available only with Modrinth priority)"
    )]
    pub fields: Option<String>,

    #[arg(
        long,
        value_name = "FORMAT",
        default_value = "table",
        help = "json | table | plain"
    )]
    pub format: String,

    #[command(flatten)]
    pub common: RpCommonArgs,
}

pub async fn handler(args: ListArgs) {
    let dir = match resolve_dir(&args.common) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    const DEFAULT_FIELDS: &str = "filename,pack_format,description";

    let fields_spec = match args.fields.as_deref() {
        None => Some(DEFAULT_FIELDS.to_string()),
        Some("all") => None,
        Some(s) => Some(s.to_string()),
    };
    let fields = match fields_spec.as_deref().map(rp::parse_fields).transpose() {
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
        let list = match rp::scan_priority_dir(&dir, &priority, &fallback).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{}: {}", Status::Err.as_colored_str(), e);
                return;
            }
        };
        if list.is_empty() {
            println!("no resource packs found.");
            return;
        }
        let value = match fields {
            Some(f) => serde_json::Value::Array(
                list.iter()
                    .map(|i| serde_json::Value::Object(rp::select_fields(i, &f)))
                    .collect(),
            ),
            None => serde_json::to_value(&list).unwrap_or_default(),
        };
        if !matches!(format, rp::Format::Json) {
            println!("\n--- resourcepacks list (items: {}) ---", list.len());
        }
        println!("{}", rp::render_value(&value, format));
        if !matches!(format, rp::Format::Json) {
            println!();
        }
        return;
    }

    match rp::scan_as(&dir, fields.as_deref(), format) {
        Ok(out) => {
            if out.trim().is_empty() {
                println!("no resource packs found.");
                return;
            }
            if !matches!(format, rp::Format::Json) {
                let count = rp::scan_dir(&dir).map(|l| l.len()).unwrap_or(0);
                println!("\n--- resourcepacks list (items: {count}) ---");
                println!("{out}");
                println!();
            } else {
                println!("{out}");
            }
        }
        Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
    }
}

pub(crate) fn resolve_dir(common: &RpCommonArgs) -> Result<PathBuf, String> {
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
        return Ok(rp::resourcepacks_dir(g));
    }
    Err("either --game-path, --dir or --file is required".to_string())
}

pub(crate) fn parse_priority(common: &RpCommonArgs) -> Result<rp::Priority, String> {
    match &common.priority {
        Some(p) => rp::Priority::parse(p).map_err(|e| e.to_string()),
        None => Ok(rp::Priority::default()),
    }
}

pub(crate) fn parse_fallback(common: &RpCommonArgs) -> Result<rp::FallbackPolicy, String> {
    if common.skip {
        return Ok(rp::FallbackPolicy::Skip);
    }
    if let Some(f) = &common.fallback_fields {
        let fields = rp::parse_fields(f).map_err(|e| e.to_string())?;
        return Ok(rp::FallbackPolicy::Fields(fields));
    }
    Ok(rp::FallbackPolicy::default())
}
