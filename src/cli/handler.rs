use crate::cli::commands::modloaders::ModloadersCommands;
use crate::cli::commands::modloaders::fabric::FabricCommands;
use crate::cli::commands::modloaders::neoforge::NeoforgeCommands;
use crate::cli::commands::{self, Commands, versions::VersionsCommands, login::LoginCommands};
use crate::cli::Cli;
use clap::Parser;

pub async fn handle_cli() -> bool {
    let args = Cli::parse();

    if args.version {
        println!(
            "{} {}\n{} {}\n{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            rmlib::NAME,
            rmlib::VERSION,
            musutils::NAME,
            musutils::VERSION
        );
        return true;
    }

    if let Some(command) = args.command {
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
                        (args.show_id, args.show_release_time, args.show_time, args.show_type, args.show_url)
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
                        url
                    ).await;
                },
                VersionsCommands::Install(args) => {
                    commands::versions::install::handle(args).await;
                }
            },
            Commands::Run(args) => {
                commands::run::handler(args).await;
            },
            Commands::Login { subcommand } => match subcommand {
                LoginCommands::Ely(args) => {
                    commands::login::ely::handler(args).await;
                }
            },
            Commands::Modloaders { subcommand } => match subcommand {
                ModloadersCommands::Fabric { subcommand } => match subcommand {
                    FabricCommands::FetchList(args) => {
                        commands::modloaders::fabric::fetch_list::handler(args).await;
                    },
                    FabricCommands::Install(args) => {
                        commands::modloaders::fabric::install::handler(args).await;
                    }
                },
                ModloadersCommands::Neoforge { subcommand } => match subcommand {
                    NeoforgeCommands::FetchList(args) => {
                        commands::modloaders::neoforge::fetch_list::handler(args).await;
                    },
                    NeoforgeCommands::Install(args) => {
                        commands::modloaders::neoforge::install::handler(args).await;
                    }
                }
            }
        }
        true
    } else {
        false
    }
}
