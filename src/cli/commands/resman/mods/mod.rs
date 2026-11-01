pub mod info;
pub mod list;
pub mod toggle;

use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct ModsArgs {
    #[command(subcommand)]
    pub subcommand: ModsCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModsCommands {
    /// List installed mods
    List(list::ListArgs),
    /// Show info for one mod
    Info(info::InfoArgs),
    /// Enable a mod (remove the .disabled suffix)
    Enable(toggle::ToggleArgs),
    /// Disable a mod (append the .disabled suffix)
    Disable(toggle::ToggleArgs),
    /// Toggle a mod's enabled state
    Toggle(toggle::ToggleArgs),
}

#[derive(Args, Debug, Clone)]
pub struct ModsCommonArgs {
    #[arg(
        long = "game-path",
        value_name = "DIR",
        help = "Game directory (mods are read from <DIR>/mods)"
    )]
    pub game_path: Option<PathBuf>,

    #[arg(
        long,
        value_name = "DIR",
        help = "Explicit mods directory (overrides --game-path)"
    )]
    pub dir: Option<PathBuf>,

    #[arg(
        long,
        value_name = "PATH",
        help = "Operate on a single mod JAR instead of a directory"
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
        help = "Skip mods that no source could resolve"
    )]
    pub skip: bool,

    #[arg(
        long = "fallback-fields",
        value_name = "FIELDS",
        conflicts_with = "skip",
        help = "Comma-separated fields taken from the local JAR when no source resolved it"
    )]
    pub fallback_fields: Option<String>,

    #[arg(short = 'j', long, help = "Output raw JSON")]
    pub json: bool,
}
