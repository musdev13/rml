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
                VersionsCommands::FetchList { 
                    r#type, 
                    page, 
                    count, 
                    json, 
                    show_id, 
                    show_release_time, 
                    show_time, 
                    show_type, 
                    show_url 
                } => {
                    let any_field_selected = show_id || show_release_time || show_time || show_type || show_url;

                    let (id, r_time, time, v_type, url) = if !any_field_selected {
                        (true, true, false, true, false)
                    } else {
                        (show_id, show_release_time, show_time, show_type, show_url)
                    };

                    commands::versions::fetch_list::handle(
                        r#type, 
                        page, 
                        count, 
                        json, 
                        id, 
                        r_time, 
                        time, 
                        v_type, 
                        url
                    ).await;
                }
            },
        }
        true
    } else {
        false
    }
}
