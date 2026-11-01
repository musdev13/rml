pub mod info;
pub mod list;

use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct ResourcepacksArgs {
    #[command(subcommand)]
    pub subcommand: ResourcepacksCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ResourcepacksCommands {
    /// List installed resource packs
    List(list::ListArgs),
    /// Show info for one resource pack
    Info(info::InfoArgs),
}

#[derive(Args, Debug, Clone)]
pub struct RpCommonArgs {
    #[arg(
        long = "game-path",
        value_name = "DIR",
        help = "Game directory (resource packs are read from <DIR>/resourcepacks)"
    )]
    pub game_path: Option<PathBuf>,

    #[arg(
        long,
        value_name = "DIR",
        help = "Explicit resource packs directory (overrides --game-path)"
    )]
    pub dir: Option<PathBuf>,

    #[arg(
        long,
        value_name = "PATH",
        help = "Operate on a single resource pack ZIP instead of a directory"
    )]
    pub file: Option<PathBuf>,

    #[arg(
        long = "priority",
        value_name = "SRCS",
        help = "Comma-separated info sources: local, modrinth, modrinth,local (default: local)"
    )]
    pub priority: Option<String>,

    #[arg(
        long,
        conflicts_with = "fallback_fields",
        help = "Skip resource packs that no source could resolve"
    )]
    pub skip: bool,

    #[arg(
        long = "fallback-fields",
        value_name = "FIELDS",
        conflicts_with = "skip",
        help = "Comma-separated fields taken from the local ZIP when no source resolved it"
    )]
    pub fallback_fields: Option<String>,

    #[arg(short = 'j', long, help = "Output raw JSON")]
    pub json: bool,
}
