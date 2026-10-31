use std::path::PathBuf;

use clap::Args;
use musutils::types::Status;
use rmlib::core::config;
use rmlib::core::run::build_command;
use rmlib::core::run::run_client;

#[cfg(feature = "profiles")]
use rmlib::extra::profiles::{
    InstanceConfig, ProfileKind, instance_default_path, load_instance, load_profile_at,
    save_instance,
};

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(help = "Game version ID to run (e.g., 1.20.4)")]
    pub version_id: Option<String>,

    #[arg(
        short,
        long,
        help = "Amount of RAM allocated for JVM (e.g., 4G or 3500M)"
    )]
    pub ram: Option<String>,

    #[arg(short = 'u', long, help = "Player nickname")]
    pub username: Option<String>,

    #[arg(long, help = "Player UUID")]
    pub uuid: Option<String>,

    #[arg(long, help = "Authentication access token")]
    pub token: Option<String>,

    #[arg(long, help = "Xbox User ID")]
    pub xuid: Option<String>,

    #[cfg(feature = "ely")]
    #[arg(
        short = 'e',
        long,
        conflicts_with = "no_ely",
        help = "Enable Ely.by skin system / authentication"
    )]
    pub ely: bool,

    #[cfg(feature = "ely")]
    #[arg(
        long = "no-ely",
        conflicts_with = "ely",
        help = "Disable Ely.by for this launch"
    )]
    pub no_ely: bool,

    #[cfg(feature = "profiles")]
    #[arg(
        short = 'p',
        long,
        conflicts_with = "no_profile",
        help = "Load account data from a saved profile (ely when -e is set, ms otherwise)"
    )]
    pub profile: bool,

    #[cfg(feature = "profiles")]
    #[arg(
        long = "no-profile",
        conflicts_with = "profile",
        help = "Disable loading a profile for this launch"
    )]
    pub no_profile: bool,

    #[cfg(feature = "profiles")]
    #[arg(
        long = "auth-path",
        value_name = "DIR",
        help = "Directory holding saved profiles (default: auth_path from config)"
    )]
    pub auth_path: Option<PathBuf>,

    #[cfg(feature = "profiles")]
    #[arg(
        long,
        value_name = "DIR",
        conflicts_with = "instance_file",
        help = "Load launch settings from <DIR>/rml.json (uses DIR as game_path when unset)"
    )]
    pub instance: Option<PathBuf>,

    #[cfg(feature = "profiles")]
    #[arg(
        long = "instance-file",
        value_name = "PATH",
        conflicts_with = "instance",
        help = "Load launch settings from an explicit JSON file"
    )]
    pub instance_file: Option<PathBuf>,

    #[cfg(feature = "profiles")]
    #[arg(
        long = "instance-output",
        value_name = "PATH",
        num_args = 0..=1,
        default_missing_value = "",
        help = "Save the launch settings to a JSON file and exit (default: <game_path>/rml.json)"
    )]
    pub instance_output: Option<String>,

    #[arg(
        short = 'b',
        long,
        conflicts_with = "no_betafix",
        help = "Apply the Betacraft / ely.by fix for pre-1.6 versions"
    )]
    pub betafix: bool,

    #[arg(
        long = "no-betafix",
        conflicts_with = "betafix",
        help = "Disable the Betacraft fix for this launch"
    )]
    pub no_betafix: bool,

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

#[allow(dead_code)]
struct LaunchValues {
    version_id: String,
    ram: String,
    username: String,
    uuid: String,
    token: String,
    xuid: String,
    ely_flag: bool,
    betafix_flag: bool,
    jvm_args: Option<String>,
    game_args: Option<String>,
    versions: PathBuf,
    assets: PathBuf,
    libs: PathBuf,
    game_path: PathBuf,
    spawn_path: PathBuf,
    java: Option<PathBuf>,
    fabric: Option<String>,
    neoforge: Option<String>,
}

pub async fn handler(args: RunArgs) {
    #[cfg(feature = "profiles")]
    let resolved = resolve_with_instance(&args);
    #[cfg(not(feature = "profiles"))]
    let resolved = resolve_plain(&args);

    let Some(v) = resolved else {
        return;
    };
    do_launch(v, args.just_command).await;
}

#[cfg(not(feature = "profiles"))]
fn resolve_plain(args: &RunArgs) -> Option<LaunchValues> {
    let Some(version_id) = args.version_id.clone() else {
        eprintln!("{}: version id is required", Status::Err.as_colored_str());
        return None;
    };

    #[cfg(feature = "ely")]
    let ely_flag = args.ely;
    #[cfg(not(feature = "ely"))]
    let ely_flag = false;

    let betafix_flag = args.betafix && !args.no_betafix;

    #[cfg(feature = "fabric")]
    let fabric = args.fabric.clone();
    #[cfg(not(feature = "fabric"))]
    let fabric: Option<String> = None;

    #[cfg(feature = "neoforge")]
    let neoforge = args.neoforge.clone();
    #[cfg(not(feature = "neoforge"))]
    let neoforge: Option<String> = None;

    let versions = args
        .versions
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_versions_path()));
    let assets = args
        .assets
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_assets_path()));
    let libs = args
        .libs
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_libs_path()));
    let game_path = args
        .game_path
        .clone()
        .unwrap_or_else(|| PathBuf::from("~/.minecraft"));
    let spawn_path = args.spawn_path.clone().unwrap_or_else(|| game_path.clone());

    Some(LaunchValues {
        version_id,
        ram: args.ram.clone().unwrap_or_else(|| "4G".to_string()),
        username: args
            .username
            .clone()
            .unwrap_or_else(|| "MusPlayer".to_string()),
        uuid: args
            .uuid
            .clone()
            .unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string()),
        token: args.token.clone().unwrap_or_else(|| "null".to_string()),
        xuid: args.xuid.clone().unwrap_or_else(|| "noxuid".to_string()),
        ely_flag,
        betafix_flag,
        jvm_args: args.jvm_args.clone(),
        game_args: args.game_args.clone(),
        versions,
        assets,
        libs,
        game_path,
        spawn_path,
        java: args.java.clone(),
        fabric,
        neoforge,
    })
}

#[cfg(feature = "profiles")]
fn resolve_with_instance(args: &RunArgs) -> Option<LaunchValues> {
    let cli_ely: Option<bool> = {
        #[cfg(feature = "ely")]
        {
            if args.no_ely {
                Some(false)
            } else if args.ely {
                Some(true)
            } else {
                None
            }
        }
        #[cfg(not(feature = "ely"))]
        {
            None
        }
    };
    let cli_betafix = if args.no_betafix {
        Some(false)
    } else if args.betafix {
        Some(true)
    } else {
        None
    };
    let cli_profile = if args.no_profile {
        Some(false)
    } else if args.profile {
        Some(true)
    } else {
        None
    };

    #[cfg(feature = "fabric")]
    let cli_fabric = args.fabric.clone();
    #[cfg(not(feature = "fabric"))]
    let cli_fabric: Option<String> = None;

    #[cfg(feature = "neoforge")]
    let cli_neoforge = args.neoforge.clone();
    #[cfg(not(feature = "neoforge"))]
    let cli_neoforge: Option<String> = None;

    let mut cfg = InstanceConfig {
        version_id: args.version_id.clone(),
        ram: args.ram.clone(),
        username: args.username.clone(),
        ely: cli_ely,
        betafix: cli_betafix,
        profile: cli_profile,
        jvm_args: args.jvm_args.clone(),
        game_args: args.game_args.clone(),
        versions: args.versions.clone(),
        assets: args.assets.clone(),
        libs: args.libs.clone(),
        game_path: args.game_path.clone(),
        spawn_path: args.spawn_path.clone(),
        java: args.java.clone(),
        fabric: cli_fabric,
        neoforge: cli_neoforge,
        auth_path: args.auth_path.clone(),
    };

    if let Some(dir) = &args.instance {
        let path = instance_default_path(dir);
        match load_instance(&path) {
            Ok(base) => cfg.merge_missing_from(&base),
            Err(e) => eprintln!(
                "{}: failed to load instance {}: {}",
                Status::Warn.as_colored_str(),
                path.display(),
                e
            ),
        }
        if cfg.game_path.is_none() {
            cfg.game_path = Some(dir.clone());
        }
    } else if let Some(path) = &args.instance_file {
        match load_instance(path) {
            Ok(base) => cfg.merge_missing_from(&base),
            Err(e) => eprintln!(
                "{}: failed to load instance {}: {}",
                Status::Warn.as_colored_str(),
                path.display(),
                e
            ),
        }
    }

    if let Some(out) = &args.instance_output {
        let game_path = cfg
            .game_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("~/.minecraft"));
        let out_path = if out.is_empty() {
            instance_default_path(&game_path)
        } else {
            PathBuf::from(out)
        };
        match save_instance(&out_path, &cfg) {
            Ok(()) => println!(
                "{}: instance saved to {}",
                Status::Ok.as_colored_str(),
                out_path.display()
            ),
            Err(e) => eprintln!(
                "{}: failed to save instance: {}",
                Status::Err.as_colored_str(),
                e
            ),
        }
        return None;
    }

    let Some(version_id) = cfg.version_id.clone() else {
        eprintln!(
            "{}: no version id (pass <VERSION_ID> or use --instance)",
            Status::Err.as_colored_str()
        );
        return None;
    };

    let profile_enabled = cfg.profile.unwrap_or(false);
    let profile_ely = cfg.ely.unwrap_or(false);
    let profile = if profile_enabled {
        let lookup_username = cfg
            .username
            .clone()
            .unwrap_or_else(|| "MusPlayer".to_string());
        let kind = if profile_ely {
            ProfileKind::Ely
        } else {
            ProfileKind::Ms
        };
        match load_profile_at(cfg.auth_path.as_deref(), kind, &lookup_username) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "{}: failed to load profile for {}: {}",
                    Status::Warn.as_colored_str(),
                    lookup_username,
                    e
                );
                None
            }
        }
    } else {
        None
    };
    if profile_enabled && profile.is_none() {
        eprintln!(
            "{}: profile not found; falling back to CLI values / defaults",
            Status::Warn.as_colored_str()
        );
    }

    let username = cfg
        .username
        .clone()
        .or_else(|| profile.as_ref().map(|p| p.username.clone()))
        .unwrap_or_else(|| "MusPlayer".to_string());
    let uuid = args
        .uuid
        .clone()
        .or_else(|| profile.as_ref().and_then(|p| p.uuid.clone()))
        .unwrap_or_else(|| "00000000-0000-0000-0000-000000000000".to_string());
    let token = args
        .token
        .clone()
        .or_else(|| profile.as_ref().map(|p| p.access_token.clone()))
        .unwrap_or_else(|| "null".to_string());
    let xuid = args
        .xuid
        .clone()
        .or_else(|| profile.as_ref().and_then(|p| p.xuid.clone()))
        .unwrap_or_else(|| "noxuid".to_string());

    #[cfg(feature = "ely")]
    let ely_flag = cfg.ely.unwrap_or(false);
    #[cfg(not(feature = "ely"))]
    let ely_flag = false;

    let betafix_flag = cfg.betafix.unwrap_or(false);

    #[cfg(feature = "fabric")]
    let fabric = cfg.fabric.clone();
    #[cfg(not(feature = "fabric"))]
    let fabric: Option<String> = None;

    #[cfg(feature = "neoforge")]
    let neoforge = cfg.neoforge.clone();
    #[cfg(not(feature = "neoforge"))]
    let neoforge: Option<String> = None;

    let versions = cfg
        .versions
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_versions_path()));
    let assets = cfg
        .assets
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_assets_path()));
    let libs = cfg
        .libs
        .clone()
        .unwrap_or_else(|| PathBuf::from(config::get_libs_path()));
    let game_path = cfg
        .game_path
        .clone()
        .unwrap_or_else(|| PathBuf::from("~/.minecraft"));
    let spawn_path = cfg.spawn_path.clone().unwrap_or_else(|| game_path.clone());

    Some(LaunchValues {
        version_id,
        ram: cfg.ram.clone().unwrap_or_else(|| "4G".to_string()),
        username,
        uuid,
        token,
        xuid,
        ely_flag,
        betafix_flag,
        jvm_args: cfg.jvm_args.clone(),
        game_args: cfg.game_args.clone(),
        versions,
        assets,
        libs,
        game_path,
        spawn_path,
        java: cfg.java.clone(),
        fabric,
        neoforge,
    })
}

async fn do_launch(v: LaunchValues, just_command: bool) {
    let natives_path = v.libs.join("natives");

    let custom_jvm_args: Vec<String> = v
        .jvm_args
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let custom_game_args: Vec<String> = v
        .game_args
        .as_deref()
        .unwrap_or("")
        .split_whitespace()
        .map(str::to_string)
        .collect();

    let v_str = v.versions.to_string_lossy();
    let a_str = v.assets.to_string_lossy();
    let l_str = v.libs.to_string_lossy();
    let n_str = natives_path.to_string_lossy();
    let g_str = v.game_path.to_string_lossy();
    let s_str = v.spawn_path.to_string_lossy();
    let java_str = v.java.as_ref().map(|p| p.to_string_lossy().into_owned());

    #[cfg(feature = "fabric")]
    let fabric_version = v.fabric.as_deref();
    #[cfg(not(feature = "fabric"))]
    let fabric_version: Option<&str> = None;

    #[cfg(feature = "neoforge")]
    let neoforge_version = v.neoforge.as_deref();
    #[cfg(not(feature = "neoforge"))]
    let neoforge_version: Option<&str> = None;

    if just_command {
        match build_command(
            &v.version_id,
            fabric_version,
            neoforge_version,
            &v.ram,
            &v.username,
            &v.uuid,
            &v.xuid,
            &v.token,
            v.ely_flag,
            v.betafix_flag,
            &v_str,
            &a_str,
            &l_str,
            &n_str,
            &g_str,
            &custom_jvm_args,
            &custom_game_args,
            just_command,
        ) {
            Ok(command) => println!("{command}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    run_client(
        &v.version_id,
        fabric_version,
        neoforge_version,
        &v.ram,
        &v.username,
        &v.uuid,
        &v.xuid,
        &v.token,
        v.ely_flag,
        v.betafix_flag,
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
