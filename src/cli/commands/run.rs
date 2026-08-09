use std::path::PathBuf;

use clap::Args;
use musutils;
use rmlib::core::config;
use rmlib::core::run::build_command;
use rmlib::core::run::run_client;

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(help = "Game version ID to run (e.g., 1.20.4)")]
    pub version_id: String,

    #[arg(
        short,
        long,
        default_value = "4G",
        help = "Amount of RAM allocated for JVM (e.g., 4G or 3500M)"
    )]
    pub ram: String,

    #[arg(
        short = 'u',
        long,
        default_value = "MusPlayer",
        help = "Player nickname"
    )]
    pub username: String,

    #[arg(
        long,
        default_value = "00000000-0000-0000-0000-000000000000",
        help = "Player UUID"
    )]
    pub uuid: String,

    #[arg(
        long,
        default_value = "null",
        help = "Authentication access token"
    )]
    pub token: String,

    #[arg(
        short = 'e',
        long,
        help = "Enable Ely.by skin system / authentication"
    )]
    pub ely: bool,

    #[arg(
        short = 'b',
        long,
        help = "Apply betacraft fix for pre-1.6 versions"
    )]
    pub betafix: bool,

    #[arg(
        long = "just-command",
        help = "Print the launch command instead of starting the game"
    )]
    pub just_command: bool,

    #[arg(
        long,
        value_name = "JVM_ARGS",
        allow_hyphen_values = true,
        help = "Additional JVM arguments"
    )]
    pub jvm_args: Option<String>,

    #[arg(
        long,
        value_name = "GAME_ARGS",
        allow_hyphen_values = true,
        help = "Additional game arguments"
    )]
    pub game_args: Option<String>,

    #[arg(
        long,
        value_name = "VERSIONS_PATH",
        help = "Path to the versions directory where JSON and client JAR are stored"
    )]
    pub versions: Option<PathBuf>,

    #[arg(
        long,
        value_name = "ASSETS_PATH",
        help = "Path to the assets directory"
    )]
    pub assets: Option<PathBuf>,

    #[arg(
        long,
        value_name = "LIBS_PATH",
        help = "Path to the libraries directory"
    )]
    pub libs: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "GAME_PATH",
        help = "Path to the game directory (working directory for launch)"
    )]
    pub game_path: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "SPAWN_PATH",
        help = "Path to the directory from which the game is launched"
    )]
    pub spawn_path: Option<PathBuf>,

    #[arg(
        long,
        value_name = "FABRIC_VERSION",
        conflicts_with = "neoforge",
        help = "Specify Fabric Loader version to run"
    )]
    pub fabric: Option<String>,

    #[arg(
        long,
        value_name = "NEOFORGE_VERSION",
        conflicts_with = "fabric",
        help = "Specify NeoForge version to run"
    )]
    pub neoforge: Option<String>,
}

pub async fn handler(args: RunArgs) {
    let mut is_custom_versions = false;
    let mut is_custom_assets = false;
    let mut is_custom_libs = false;
    let mut is_custom_game = false;
    let mut is_custom_spawn = false;

    let versions_path = musutils::types::deoption(
        args.versions,
        || PathBuf::from(config::get_versions_path()),
        &mut is_custom_versions,
    );

    let assets_path = musutils::types::deoption(
        args.assets,
        || PathBuf::from(config::get_assets_path()),
        &mut is_custom_assets,
    );

    let libs_path = musutils::types::deoption(
        args.libs,
        || PathBuf::from(config::get_libs_path()),
        &mut is_custom_libs,
    );

    let game_path = musutils::types::deoption(
        args.game_path,
        || PathBuf::from("~/.minecraft"),
        &mut is_custom_game,
    );

    let spawn_path = musutils::types::deoption(
        args.spawn_path,
        || game_path.clone(),
        &mut is_custom_spawn,
    );

    let natives_path = libs_path.join("natives");

    let custom_jvm_args: Vec<String> = args
        .jvm_args
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let custom_game_args: Vec<String> = args
        .game_args
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let v_str = versions_path.to_string_lossy();
    let a_str = assets_path.to_string_lossy();
    let l_str = libs_path.to_string_lossy();
    let n_str = natives_path.to_string_lossy();
    let g_str = game_path.to_string_lossy();
    let s_str = spawn_path.to_string_lossy();

    if args.just_command {
        match build_command(
            &args.version_id,
            args.fabric.as_deref(),
            args.neoforge.as_deref(),
            &args.ram,
            &args.username,
            &args.uuid,
            &args.token,
            args.ely,
            args.betafix,
            &v_str,
            &a_str,
            &l_str,
            &n_str,
            &g_str,
            &custom_jvm_args,
            &custom_game_args,
        ) {
            Ok(command) => println!("{command}"),
            Err(e) => {
                eprintln!(
                    "{}: {}",
                    musutils::types::Status::Err.as_colored_str(),
                    e
                );
            }
        }

        return;
    }

    run_client(
        &args.version_id,
        args.fabric.as_deref(),
        args.neoforge.as_deref(),
        &args.ram,
        &args.username,
        &args.uuid,
        &args.token,
        args.ely,
        args.betafix,
        &v_str,
        &a_str,
        &l_str,
        &n_str,
        &g_str,
        &s_str,
        &custom_jvm_args,
        &custom_game_args,
    )
    .await;
}
