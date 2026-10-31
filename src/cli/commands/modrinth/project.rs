use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::project;

#[derive(Args, Debug)]
pub struct ProjectArgs {
    #[arg(help = "Project slug or ID")]
    pub id_or_slug: String,
}

pub async fn handler(args: ProjectArgs, json: bool) {
    let p = match project::get_project(&args.id_or_slug).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&p) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    let line = musutils::types::line::draw_colored('=', 35, musutils::color::Colors::Yellow);

    println!("{line}");
    println!("{} {} ({})", Status::Ok.as_colored_str(), p.title, p.slug);
    println!("{line}");
    println!("{} id:          {}", Status::Note.as_colored_str(), p.id);
    println!(
        "{} type:        {}",
        Status::Note.as_colored_str(),
        p.project_type
    );
    println!(
        "{} downloads:   {}",
        Status::Note.as_colored_str(),
        p.downloads
    );
    println!(
        "{} followers:   {}",
        Status::Note.as_colored_str(),
        p.followers
    );
    println!(
        "{} client side: {}",
        Status::Note.as_colored_str(),
        p.client_side
    );
    println!(
        "{} server side: {}",
        Status::Note.as_colored_str(),
        p.server_side
    );
    if let Some(l) = &p.license {
        println!(
            "{} license:     {} ({})",
            Status::Note.as_colored_str(),
            l.name,
            l.id
        );
    }
    println!(
        "{} loaders:     {}",
        Status::Note.as_colored_str(),
        p.loaders.join(", ")
    );
    println!(
        "{} game vers:   {}",
        Status::Note.as_colored_str(),
        p.game_versions.join(", ")
    );
    if !p.description.is_empty() {
        println!(
            "{} description: {}",
            Status::Note.as_colored_str(),
            p.description
        );
    }
    println!("{line}");
}
