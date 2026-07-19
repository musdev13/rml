use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InstallArgs {
    #[arg(
        help = "Game version ID to install (e.g., 1.20.4)",
        conflicts_with = "json"
    )]
    pub version_id: Option<String>,

    #[arg(
        short = 'j',
        long,
        value_name = "PATH",
        help = "Path to a locally downloaded version JSON file",
        conflicts_with = "version_id"
    )]
    pub json: Option<PathBuf>,

    #[arg(short, long, value_name = "PATH")]
    pub directory: Option<PathBuf>,

    #[arg(short, long, value_name = "LIBS_PATH")]
    pub libs: Option<PathBuf>,

    #[arg(short, long, value_name = "ASSETS_PATH")]
    pub assets: Option<PathBuf>,

    #[arg(short, long, help = "Perform a soft installation")]
    pub soft: bool,

    #[arg(long, help = "Install client")]
    pub iclient: bool,

    #[arg(long, help = "Install libraries")]
    pub ilibs: bool,

    #[arg(long, help = "Install assets")]
    pub iassets: bool,
}

pub async fn handle(args: InstallArgs) {
    if args.version_id.is_none() && args.json.is_none() {
        eprintln!(
            "{}: you must provide either a version ID or a local JSON path via --json",
            musutils::types::Status::Err.as_colored_str()
        );
        std::process::exit(1);
    }

    let (iclient, ilibs, iassets) = if !args.iclient && !args.ilibs && !args.iassets {
        (true, true, true)
    } else {
        (args.iclient, args.ilibs, args.iassets)
    };

    rmlib::core::version::install::install(
        args.version_id,
        args.json,
        args.directory,
        args.libs,
        args.assets,
        args.soft,
        iclient,
        ilibs,
        iassets,
    )
    .await;
}
