use clap::Args;
use std::path::PathBuf;

use crate::core;

#[derive(Args, Debug)]
pub struct InstallArgs {
    pub version_id: String,
    #[arg(short, long, value_name = "PATH")]
    pub directory: Option<PathBuf>,
}

pub async fn handle(version_id: String, directory: Option<PathBuf>) {
    println!("{}, {}", version_id, directory.clone().map_or("default".to_string(), |p| p.display().to_string()));
    core::version::install(version_id, directory).await;
}
