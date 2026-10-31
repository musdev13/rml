use clap::Args;
use musutils::types::Status;

use rmlib::extra::modrinth::version;

#[derive(Args, Debug)]
pub struct VersionArgs {
    #[arg(help = "Version ID")]
    pub version_id: String,
}

pub async fn handler(args: VersionArgs, json: bool) {
    let v = match version::get_version(&args.version_id).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}: {}", Status::Err.as_colored_str(), e);
            return;
        }
    };

    if json {
        match serde_json::to_string_pretty(&v) {
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
        v.name,
        v.version_number
    );
    println!("{line}");
    println!("{} id:        {}", Status::Note.as_colored_str(), v.id);
    println!(
        "{} project:   {}",
        Status::Note.as_colored_str(),
        v.project_id
    );
    println!(
        "{} type:      {}",
        Status::Note.as_colored_str(),
        v.version_type
    );
    println!(
        "{} published: {}",
        Status::Note.as_colored_str(),
        v.date_published
    );
    println!(
        "{} mc:        {}",
        Status::Note.as_colored_str(),
        v.game_versions.join(", ")
    );
    println!(
        "{} loaders:   {}",
        Status::Note.as_colored_str(),
        v.loaders.join(", ")
    );
    if let Some(f) = v.primary_file() {
        println!(
            "{} file:      {} ({} bytes)",
            Status::Note.as_colored_str(),
            f.filename,
            f.size
        );
    }
    if !v.dependencies.is_empty() {
        println!(
            "{} deps:      {}",
            Status::Note.as_colored_str(),
            v.dependencies.len()
        );
    }
    println!("{line}");
}
