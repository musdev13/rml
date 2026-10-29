use clap::Args;
use musutils::types::Status;

#[derive(Args, Debug)]
pub struct LoginArgs {
    #[arg(short = 'j', long = "json", help = "Print the result as JSON")]
    pub json: bool,
}

pub async fn handler(args: LoginArgs, auth_path: Option<std::path::PathBuf>) {
    let tokens = match rmlib::core::auth::login_at(auth_path.as_deref()).await {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "{}: Microsoft authentication failed: {}",
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

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    println!(
        "{} Microsoft authentication successful",
        Status::Ok.as_colored_str()
    );
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
