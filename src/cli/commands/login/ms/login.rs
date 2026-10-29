use clap::Args;
use musutils::types::Status;

use super::confirm::wait_for_key;

#[derive(Args, Debug)]
pub struct LoginArgs {
    #[arg(short = 'j', long = "json", help = "Print the result as JSON")]
    pub json: bool,
}

pub async fn handler(args: LoginArgs, auth_path: Option<std::path::PathBuf>, save: bool) {
    let device = match rmlib::core::auth::request_device_code().await {
        Ok(d) => d,
        Err(e) => {
            eprintln!(
                "{}: failed to start Microsoft authentication: {}",
                Status::Err.as_colored_str(),
                e
            );
            return;
        }
    };

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    eprintln!("{line}");
    eprintln!(
        "{} Open the page and enter the code",
        Status::Inf.as_colored_str()
    );
    eprintln!("{line}");
    eprintln!(
        "{} Page: {}",
        Status::Note.as_colored_str(),
        device.verification_uri
    );
    eprintln!(
        "{} Code: {}",
        Status::Note.as_colored_str(),
        device.user_code
    );
    eprintln!("{line}");
    eprintln!("Waiting for confirmation...");

    let tokens = match rmlib::core::auth::poll_device_code(auth_path.as_deref(), &device).await {
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

    #[cfg(feature = "profiles")]
    if save {
        save_ms_profile(&tokens, auth_path.as_deref()).await;
    }
    #[cfg(not(feature = "profiles"))]
    let _ = save;

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

#[cfg(feature = "profiles")]
async fn save_ms_profile(
    tokens: &rmlib::core::auth::MsTokens,
    auth_path: Option<&std::path::Path>,
) {
    use rmlib::extra::profiles::{Profile, ProfileKind, save_profile_at};

    match rmlib::core::auth::get_profile(&tokens.access_token).await {
        Ok(profile_data) => {
            let profile = Profile {
                username: profile_data.username.clone(),
                uuid: Some(profile_data.uuid.clone()),
                access_token: profile_data.access_token.clone(),
                client_token: None,
                xuid: Some(profile_data.xuid.clone()),
                has_license: Some(profile_data.has_license),
            };
            match save_profile_at(auth_path, ProfileKind::Ms, &profile) {
                Ok(path) => eprintln!(
                    "{}: profile saved to {}",
                    Status::Ok.as_colored_str(),
                    path.display()
                ),
                Err(e) => eprintln!(
                    "{}: failed to save profile: {}",
                    Status::Err.as_colored_str(),
                    e
                ),
            }
        }
        Err(e) => eprintln!(
            "{}: failed to resolve profile for saving: {}",
            Status::Warn.as_colored_str(),
            e
        ),
    }
}
