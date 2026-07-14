use crate::cli::{commands,commands::{Commands,versions::VersionsCommands}};
use crate::cli::Cli;
use clap::Parser;

pub async fn handle_cli() -> bool {
    let args = Cli::parse();

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
                    commands::versions::install::handle(args.version_id, args.directory).await;
                }
            },
        }
        true
    } else {
        false
    }
}
