use clap::Args;
use musutils::types::Status;

use super::confirm::wait_for_key;

#[derive(Args, Debug)]
pub struct RefreshArgs {
    #[arg(short = 'j', long = "json", help = "Print the result as JSON")]
    pub json: bool,
}

pub async fn handler(args: RefreshArgs, auth_path: Option<std::path::PathBuf>) {
    let saved = match rmlib::core::auth::load_tokens_at(auth_path.as_deref()) {
        Ok(Some(t)) => t,
        Ok(None) => {
            eprintln!(
                "{}: no saved Microsoft tokens; run `rml auth ms login` first",
                Status::Err.as_colored_str()
            );
            return;
        }
        Err(e) => {
            eprintln!(
                "{}: failed to read saved tokens: {}",
                Status::Err.as_colored_str(),
                e
            );
            return;
        }
    };

    let tokens =
        match rmlib::core::auth::refresh_at(auth_path.as_deref(), &saved.refresh_token).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "{}: failed to refresh Microsoft tokens: {}",
                    Status::Err.as_colored_str(),
                    e
                );
                return;
            }
        };

    if args.json {
        match serde_json::to_string_pretty(&tokens) {
            Ok(pretty) => println!("{pretty}"),
            Err(e) => eprintln!(
                "{}: failed to serialize auth result: {}",
                Status::Err.as_colored_str(),
                e
            ),
        }
        return;
    }

    wait_for_key("Press any key to show authentication data (personal tokens will be printed)...");

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    println!("{} Microsoft tokens refreshed", Status::Ok.as_colored_str());
    println!("{line}");
    println!(
        "{} Access token: {}",
        Status::Note.as_colored_str(),
        tokens.access_token
    );
    println!(
        "{} Refresh token: {}",
        Status::Note.as_colored_str(),
        tokens.refresh_token
    );
    println!("{line}");
}
