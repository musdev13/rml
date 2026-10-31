use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::user;

#[derive(Args, Debug)]
pub struct UserArgs {
    #[arg(help = "User ID or username")]
    pub id_or_username: String,
}

pub async fn handler(args: UserArgs, json: bool) {
    let u = match user::get_user(&args.id_or_username).await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&u) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    println!(
        "{} {} ({})",
        Status::Ok.as_colored_str(),
        u.name.clone().unwrap_or_else(|| u.username.clone()),
        u.username
    );
    println!("{line}");
    println!("{} id:      {}", Status::Note.as_colored_str(), u.id);
    if let Some(bio) = &u.bio
        && !bio.is_empty()
    {
        println!("{} bio:     {}", Status::Note.as_colored_str(), bio);
    }
    println!("{line}");
}
