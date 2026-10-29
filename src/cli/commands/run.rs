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

    #[arg(long, default_value = "null", help = "Authentication access token")]
    pub token: String,

    #[arg(long, default_value = "noxuid", help = "Xbox User ID")]
    pub xuid: String,

    #[cfg(feature = "ely")]
    #[arg(short = 'e', long, help = "Enable Ely.by skin system / authentication")]
    pub ely: bool,

    #[arg(short = 'b', long, help = "Apply betacraft fix for pre-1.6 versions")]
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
        value_name = "JAVA_PATH",
        help = "Path to the Java executable to use (defaults to `java` from PATH)"
    )]
    pub java: Option<PathBuf>,

    #[cfg(feature = "fabric")]
    #[cfg_attr(feature = "neoforge", arg(conflicts_with = "neoforge"))]
    #[arg(
        long,
        value_name = "FABRIC_VERSION",
        help = "Specify Fabric Loader version to run"
    )]
    pub fabric: Option<String>,

    #[cfg(feature = "neoforge")]
    #[cfg_attr(feature = "fabric", arg(conflicts_with = "fabric"))]
    #[arg(
        long,
        value_name = "NEOFORGE_VERSION",
        help = "Specify NeoForge version to run"
    )]
    pub neoforge: Option<String>,
}

pub async fn handler(args: RunArgs) {
    let versions_path = args
        .versions
        .unwrap_or_else(|| PathBuf::from(config::get_versions_path()));
    let assets_path = args
        .assets
        .unwrap_or_else(|| PathBuf::from(config::get_assets_path()));
    let libs_path = args
        .libs
        .unwrap_or_else(|| PathBuf::from(config::get_libs_path()));
    let game_path = args
        .game_path
        .unwrap_or_else(|| PathBuf::from("~/.minecraft"));
    let spawn_path = args.spawn_path.unwrap_or_else(|| game_path.clone());

    #[cfg(feature = "ely")]
    let ely_flag = args.ely;
    #[cfg(not(feature = "ely"))]
    let ely_flag = false;

    #[cfg(feature = "fabric")]
    let fabric_version = args.fabric.as_deref();
    #[cfg(not(feature = "fabric"))]
    let fabric_version: Option<&str> = None;

    #[cfg(feature = "neoforge")]
    let neoforge_version = args.neoforge.as_deref();
    #[cfg(not(feature = "neoforge"))]
    let neoforge_version: Option<&str> = None;

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
    let java_str = args.java.as_ref().map(|p| p.to_string_lossy().into_owned());

    if args.just_command {
        match build_command(
            &args.version_id,
            fabric_version,
            neoforge_version,
            &args.ram,
            &args.username,
            &args.uuid,
            &args.xuid,
            &args.token,
            ely_flag,
            args.betafix,
            &v_str,
            &a_str,
            &l_str,
            &n_str,
            &g_str,
            &custom_jvm_args,
            &custom_game_args,
            args.just_command,
        ) {
            Ok(command) => println!("{command}"),
            Err(e) => {
                eprintln!("{}: {}", musutils::types::Status::Err.as_colored_str(), e);
            }
        }

        return;
    }

    run_client(
        &args.version_id,
        fabric_version,
        neoforge_version,
        &args.ram,
        &args.username,
        &args.uuid,
        &args.xuid,
        &args.token,
        ely_flag,
        args.betafix,
        &v_str,
        &a_str,
        &l_str,
        &n_str,
        &g_str,
        &s_str,
        java_str.as_deref(),
        &custom_jvm_args,
        &custom_game_args,
    )
    .await;
}
