use musutils::types::Status;

pub fn warn_if_invalid_config() {
    if let Err(e) = rmlib::core::config::check_config() {
        eprintln!(
            "{}: config is invalid ({}); continuing anyway",
            Status::Warn.as_colored_str(),
            e
        );
    }
}
