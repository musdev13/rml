use crate::cli::Cli;
use crate::cli::commands::login::LoginCommands;
use crate::cli::commands::login::ms::MsCommands;
#[cfg(any(feature = "fabric", feature = "neoforge"))]
use crate::cli::commands::modloaders::ModloadersCommands;
#[cfg(feature = "fabric")]
use crate::cli::commands::modloaders::fabric::FabricCommands;
#[cfg(feature = "neoforge")]
use crate::cli::commands::modloaders::neoforge::NeoforgeCommands;
use crate::cli::commands::{self, Commands, versions::VersionsCommands};
use clap::Parser;

pub async fn handle_cli() -> bool {
    let args = Cli::parse();

    if args.version {
        let rmlib_mods = rmlib::available_modules()
            .iter()
            .map(|m| m.name)
            .collect::<Vec<_>>()
            .join(", ");
        let musutils_mods = musutils::available_modules()
            .iter()
            .map(|m| format!("{} v{}", m.name, m.generation))
            .collect::<Vec<_>>()
            .join(", ");

        println!(
            "{} {}\n{} {} ({})\n{} {} ({})\nv{}-{}-{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            rmlib::NAME,
            rmlib::VERSION,
            rmlib_mods,
            musutils::NAME,
            musutils::VERSION,
            musutils_mods,
            env!("CARGO_PKG_VERSION"),
            rmlib::VERSION,
            musutils::VERSION,
        );
        return true;
    }

    if let Some(command) = args.command {
        crate::cli::preflight::warn_if_invalid_config();

        match command {
            Commands::Test => {
                commands::test::handle();
            }
            Commands::Init { force } => {
                commands::init::handle(force);
            }
            Commands::Versions { subcommand } => match subcommand {
                VersionsCommands::FetchList(args) => {
                    let any_field_selected = args.show_id
                        || args.show_release_time
                        || args.show_time
                        || args.show_type
                        || args.show_url;

                    let (id, r_time, time, v_type, url) = if !any_field_selected {
                        (true, true, false, true, false)
                    } else {
                        (
                            args.show_id,
                            args.show_release_time,
                            args.show_time,
                            args.show_type,
                            args.show_url,
                        )
                    };

                    commands::versions::fetch_list::handle(
                        args.r#type,
                        args.page,
                        args.count,
                        args.json,
                        id,
                        r_time,
                        time,
                        v_type,
                        url,
                    )
                    .await;
                }
                VersionsCommands::Install(args) => {
                    commands::versions::install::handle(args).await;
                }
            },
            Commands::Run(args) => {
                commands::run::handler(args).await;
            }
            Commands::Login { subcommand } => match subcommand {
                #[cfg(feature = "ely")]
                LoginCommands::Ely(args) => {
                    commands::login::ely::handler(args).await;
                }
                LoginCommands::Ms(ms) => {
                    #[cfg(feature = "profiles")]
                    let save = ms.save;
                    #[cfg(not(feature = "profiles"))]
                    let save = false;
                    match ms.subcommand {
                        MsCommands::Login(args) => {
                            commands::login::ms::login::handler(args, ms.auth_path, save).await;
                        }
                        MsCommands::Refresh(args) => {
                            commands::login::ms::refresh::handler(args, ms.auth_path).await;
                        }
                        MsCommands::Data(args) => {
                            commands::login::ms::data::handler(args, ms.auth_path, save).await;
                        }
                        MsCommands::Logout(args) => {
                            commands::login::ms::logout::handler(args, ms.auth_path);
                        }
                    }
                }
            },
            #[cfg(any(feature = "fabric", feature = "neoforge"))]
            Commands::Modloaders { subcommand } => match subcommand {
                #[cfg(feature = "fabric")]
                ModloadersCommands::Fabric { subcommand } => match subcommand {
                    FabricCommands::FetchList(args) => {
                        commands::modloaders::fabric::fetch_list::handler(args).await;
                    }
                    FabricCommands::Install(args) => {
                        commands::modloaders::fabric::install::handler(args).await;
                    }
                },
                #[cfg(feature = "neoforge")]
                ModloadersCommands::Neoforge { subcommand } => match subcommand {
                    NeoforgeCommands::FetchList(args) => {
                        commands::modloaders::neoforge::fetch_list::handler(args).await;
                    }
                    NeoforgeCommands::Install(args) => {
                        commands::modloaders::neoforge::install::handler(args).await;
                    }
                },
            },
        }
        true
    } else {
        false
    }
}
