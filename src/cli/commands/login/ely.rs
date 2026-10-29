use clap::Args;
use crossterm::event::{self, Event};
use std::io::{self, Write};
use std::path::PathBuf;

use musutils::types::Status;

#[derive(Args, Debug)]
pub struct ElyArgs {
    #[arg(short = 'a', long = "ask", conflicts_with_all = ["login", "passwd"])]
    pub ask: bool,

    #[arg(
        short = 'l',
        long = "login",
        required_unless_present_any = ["ask", "patch"]
    )]
    pub login: Option<String>,

    #[arg(
        short = 'p',
        long = "passwd",
        required_unless_present_any = ["ask", "patch"]
    )]
    pub passwd: Option<String>,

    #[arg(long = "patch")]
    pub patch: bool,

    #[arg(long, value_name = "LIBS_PATH", requires = "patch")]
    pub libs: Option<PathBuf>,

    #[arg(short = 'j', long = "json", conflicts_with_all = ["ask"])]
    pub json: bool,

    #[cfg(feature = "profiles")]
    #[arg(long, help = "Save the account as a profile")]
    pub save: bool,

    #[cfg(feature = "profiles")]
    #[arg(
        long = "auth-path",
        value_name = "DIR",
        help = "Directory holding saved profiles (default: auth_path from config)"
    )]
    pub auth_path: Option<PathBuf>,
}

fn prompt_login() -> io::Result<String> {
    print!("login: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_passwd() -> io::Result<String> {
    rpassword::prompt_password("passwd: ")
}

pub async fn handler(args: ElyArgs) {
    let (login, passwd) = if args.ask {
        let login = match prompt_login() {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "{}: failed to read login: {}",
                    Status::Err.as_colored_str(),
                    e
                );
                return;
            }
        };
        let passwd = match prompt_passwd() {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "{}: failed to read password: {}",
                    Status::Err.as_colored_str(),
                    e
                );
                return;
            }
        };
        (login, passwd)
    } else if let (Some(login), Some(passwd)) = (args.login, args.passwd) {
        (login, passwd)
    } else {
        (String::new(), String::new())
    };

    if args.patch
        && let Err(e) = rmlib::extra::ely::install_patch(args.libs, args.json).await
    {
        eprintln!(
            "{}: failed to install authlib patch: {}",
            Status::Err.as_colored_str(),
            e
        );
        return;
    }

    if login.is_empty() {
        return;
    }

    let res = match rmlib::extra::ely::login(login, passwd).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "{}: Ely.by authentication failed: {}",
                Status::Err.as_colored_str(),
                e
            );
            return;
        }
    };

    #[cfg(feature = "profiles")]
    if args.save {
        use rmlib::extra::profiles::{Profile, ProfileKind, save_profile_at};
        let profile = Profile {
            username: res.profile_name.clone(),
            uuid: Some(res.profile_id.clone()),
            access_token: res.access_token.clone(),
            client_token: Some(res.client_token.clone()),
            xuid: None,
            has_license: None,
        };
        match save_profile_at(args.auth_path.as_deref(), ProfileKind::Ely, &profile) {
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

    if args.json {
        match serde_json::to_string_pretty(&res) {
            Ok(pretty) => println!("{pretty}"),
            Err(e) => eprintln!(
                "{}: failed to serialize auth result: {}",
                Status::Err.as_colored_str(),
                e
            ),
        }
        return;
    }

    if args.ask {
        println!();
        println!("Press any key to show authentication data...");
        if let Err(e) = io::stdout().flush() {
            eprintln!(
                "{}: failed to flush stdout: {}",
                Status::Err.as_colored_str(),
                e
            );
            return;
        }
        loop {
            if let Ok(Event::Key(_)) = event::read() {
                break;
            }
        }
        println!();
    }

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    println!(
        "{} Ely.by authentication successful",
        Status::Ok.as_colored_str()
    );
    println!("{line}");

    println!(
        "{} Access token: {}",
        Status::Note.as_colored_str(),
        res.access_token
    );
    println!(
        "{} Client token: {}",
        Status::Note.as_colored_str(),
        res.client_token
    );
    println!(
        "{} Profile ID: {}",
        Status::Note.as_colored_str(),
        res.profile_id
    );
    println!(
        "{} Profile name: {}",
        Status::Note.as_colored_str(),
        res.profile_name
    );

    println!("{line}");
}
