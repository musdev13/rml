use musutils::types::Status;

use rmlib::extra::modrinth::dto::InstallResult;

pub fn print_result(result: &InstallResult, json: bool) {
    if json {
        match serde_json::to_string_pretty(result) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{}: {}", Status::Err.as_colored_str(), e),
        }
        return;
    }

    println!(
        "{}: installed {} file(s)",
        Status::Ok.as_colored_str(),
        result.installed.len()
    );
    for f in &result.installed {
        println!("- {} ({} bytes)", f.path.display(), f.bytes);
    }

    for s in &result.skipped {
        eprintln!(
            "{}: skipped required dependency {} ({})",
            Status::Warn.as_colored_str(),
            s.project_id,
            s.reason.as_str()
        );
    }
}
