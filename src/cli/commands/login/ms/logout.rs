use clap::Args;
use musutils::types::Status;

#[derive(Args, Debug)]
pub struct LogoutArgs {
    #[arg(short = 'j', long = "json", help = "Print the result as JSON")]
    pub json: bool,
}

pub fn handler(args: LogoutArgs, auth_path: Option<std::path::PathBuf>) {
    if let Err(e) = rmlib::core::auth::clear_tokens_at(auth_path.as_deref()) {
        eprintln!(
            "{}: failed to remove saved Microsoft tokens: {}",
            Status::Err.as_colored_str(),
            e
        );
        return;
    }

    if args.json {
        println!("{{\"ok\": true}}");
        return;
    }

    println!("{}: Microsoft tokens removed", Status::Ok.as_colored_str());
}
