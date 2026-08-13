use clap::Args;
use crossterm::event::{self, Event};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ElyArgs {
    #[arg(
        short = 'a',
        long = "ask",
        conflicts_with_all = ["login", "passwd"]
    )]
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

    #[arg(
        long,
        value_name = "LIBS_PATH",
        requires = "patch",
    )]
    pub libs: Option<PathBuf>,

    #[arg(short = 'j', long = "json", conflicts_with_all = ["ask"])]
    pub json: bool,
}

pub async fn handler(args: ElyArgs) {
    let (login, passwd) = if args.ask {
        print!("login: ");
        io::stdout().flush().unwrap();

        let mut input_login = String::new();
        io::stdin().read_line(&mut input_login).unwrap();

        let input_passwd = rpassword::prompt_password("passwd: ").unwrap();

        (input_login.trim().to_string(), input_passwd)
    } else if let (Some(login), Some(passwd)) = (args.login, args.passwd) {
        (login, passwd)
    } else {
        (String::new(), String::new())
    };

    if args.patch {
        rmlib::core::auth::ely::install_patch(args.libs, args.json)
            .await
            .expect("failed to install authlib patch");
    }

    if !login.is_empty() {
        let res = rmlib::core::auth::ely::login(login, passwd)
            .await
            .expect("can't fetch profile data");

        if args.json {
            let pretty_json = serde_json::to_string_pretty(&res)
                .expect("failed to serialize to json");

            println!("{pretty_json}");
        } else {
            if args.ask {
                println!();
                println!("Press any key to show authentication data...");
                io::stdout().flush().unwrap();

                loop {
                    if let Ok(Event::Key(_)) = event::read() {
                        break;
                    }
                }

                println!();
            }

            let line = musutils::types::line::draw_colored(
                '=',
                35,
                musutils::color::Colors::Yellow,
            );

            println!("{line}");
            println!(
                "{} Ely.by authentication successful",
                musutils::types::Status::Ok.as_colored_str()
            );
            println!("{line}");

            println!(
                "{} Access token: {}",
                musutils::types::Status::Note.as_colored_str(),
                res.access_token
            );
            println!(
                "{} Client token: {}",
                musutils::types::Status::Note.as_colored_str(),
                res.client_token
            );
            println!(
                "{} Profile ID: {}",
                musutils::types::Status::Note.as_colored_str(),
                res.profile_id
            );
            println!(
                "{} Profile name: {}",
                musutils::types::Status::Note.as_colored_str(),
                res.profile_name
            );

            println!("{line}");
        }
    }
}
