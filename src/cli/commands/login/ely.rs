use clap::Args;
use std::io::{self, Write};

#[derive(Args, Debug)]
pub struct ElyArgs {
    #[arg(short = 'a', long = "ask", conflicts_with_all = ["login", "passwd"])]
    pub ask: bool,

    #[arg(short = 'l', long = "login", required_unless_present = "ask")]
    pub login: Option<String>,

    #[arg(short = 'p', long = "passwd", required_unless_present = "ask")]
    pub passwd: Option<String>,
}

pub async fn handler(args: ElyArgs){
    let (login, passwd) = if args.ask {
        print!("login: ");
        io::stdout().flush().unwrap();
        let mut input_login = String::new();
        io::stdin().read_line(&mut input_login).unwrap();

        let input_passwd = rpassword::prompt_password("passwd: ").unwrap();

        (input_login.trim().to_string(), input_passwd)
    } else {
        (
            args.login.expect("Login must be specified"),
            args.passwd.expect("Password must be specified"),
        )
    };

    let res = rmlib::core::auth::ely::login(login, passwd).await.expect("can't fetch profile data");
    let pretty_json = serde_json::to_string_pretty(&res).expect("failed to serialize to json");
    println!("{}", pretty_json);
}
