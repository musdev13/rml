use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InstallArgs {
    pub version_id: String,
    #[arg(short, long, value_name = "PATH")]
    pub directory: Option<PathBuf>,
    #[arg(short, long, value_name = "LIBS_PATH")]
    pub libs: Option<PathBuf>,
    #[arg(short, long, value_name = "ASSETS_PATH")]
    pub assets: Option<PathBuf>,
}

pub async fn handle(version_id: String, directory: Option<PathBuf>, libs: Option<PathBuf>, assets: Option<PathBuf>) {
    rmlib::core::version::install(version_id, directory, libs, assets).await;
}
