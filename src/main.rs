// Copyright (c) 2026 Mus. Licensed under the MIT License.
// See LICENSE file for full license text.

mod tui;
mod cli;

#[tokio::main]
async fn main() {
    if !cli::handle_cli().await {
        tui::run_tui();
    }
}
