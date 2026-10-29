pub mod data;
pub mod login;
pub mod logout;
pub mod refresh;

use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct MsArgs {
    #[arg(
        long = "auth-path",
        global = true,
        value_name = "DIR",
        help = "Directory holding ms.json (default: auth_path from config)"
    )]
    pub auth_path: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: MsCommands,
}

#[derive(Subcommand, Debug)]
pub enum MsCommands {
    Login(login::LoginArgs),
    Refresh(refresh::RefreshArgs),
    Data(data::DataArgs),
    Logout(logout::LogoutArgs),
}
