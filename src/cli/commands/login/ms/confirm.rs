use std::io::{self, Write};

use crossterm::event::{self, Event};
use musutils::types::Status;

pub fn wait_for_key(message: &str) {
    println!();
    println!("{message}");
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
