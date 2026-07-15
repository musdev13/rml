use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct InstallArgs {
    pub version_id: String,
    #[arg(short, long, value_name = "PATH")]
    pub directory: Option<PathBuf>,
    #[arg(short, long, value_name = "LIBS_PATH")]
    pub libs: Option<PathBuf>,
}

pub async fn handle(version_id: String, directory: Option<PathBuf>, libs: Option<PathBuf>) {
    println!("{}\n{}\n{}\n{}", version_id, "-".repeat(10), directory.clone().map_or("default".to_string(), |p| p.display().to_string()), libs.clone().map_or("default".to_string(), |p| p.display().to_string()));
    rmlib::core::version::install(version_id, directory, libs).await;
}
