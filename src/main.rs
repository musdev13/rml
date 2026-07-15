mod tui;
mod cli;

#[tokio::main]
async fn main() {
    if !cli::handle_cli().await {
        tui::run_tui();
    }
}
