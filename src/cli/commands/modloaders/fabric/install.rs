use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InstallArgs {
    pub version_id: String,
    pub fabric_version: String,

    #[arg(short, long, value_name = "VERSIONS_PATH")]
    pub versions: Option<PathBuf>,

    #[arg(short, long, value_name = "LIBS_PATH")]
    pub libs: Option<PathBuf>,

    #[arg(short, long, value_name = "ASSETS_PATH")]
    pub assets: Option<PathBuf>,
}

pub async fn handler(args: InstallArgs) {
    rmlib::core::modloaders::fabric::install(
        args.version_id,
        args.fabric_version,
        args.versions,
        args.libs,
        args.assets,
    )
    .await;
}
