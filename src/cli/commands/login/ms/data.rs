use clap::Args;
use musutils::types::Status;

#[derive(Args, Debug)]
pub struct DataArgs {
    #[arg(short = 'j', long = "json", help = "Print the result as JSON")]
    pub json: bool,
}

pub async fn handler(args: DataArgs, auth_path: Option<std::path::PathBuf>) {
    let profile = match rmlib::core::auth::get_data_at(auth_path.as_deref()).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "{}: failed to get Minecraft profile: {}",
                Status::Err.as_colored_str(),
                e
            );
            return;
        }
    };

    if !profile.has_license {
        eprintln!(
            "{}: account has no Minecraft license; license-dependent fields are placeholders",
            Status::Warn.as_colored_str()
        );
    }

    if args.json {
        match serde_json::to_string_pretty(&profile) {
            Ok(pretty) => println!("{pretty}"),
            Err(e) => eprintln!(
                "{}: failed to serialize profile: {}",
                Status::Err.as_colored_str(),
                e
            ),
        }
        return;
    }

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    if profile.has_license {
        println!(
            "{} Minecraft profile retrieved",
            Status::Ok.as_colored_str()
        );
    } else {
        println!(
            "{} Minecraft profile retrieved (no license)",
            Status::Warn.as_colored_str()
        );
    }
    println!("{line}");
    println!(
        "{} Username:     {}",
        Status::Note.as_colored_str(),
        profile.username
    );
    println!(
        "{} UUID:         {}",
        Status::Note.as_colored_str(),
        profile.uuid
    );
    println!(
        "{} Access token: {}",
        Status::Note.as_colored_str(),
        profile.access_token
    );
    println!(
        "{} XUID:         {}",
        Status::Note.as_colored_str(),
        profile.xuid
    );
    println!("{line}");
}
