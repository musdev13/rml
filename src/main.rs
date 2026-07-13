mod core;
mod tui;
mod cli;

fn main() {
    if !cli::handle_cli() {
        tui::run_tui();
    }
}
