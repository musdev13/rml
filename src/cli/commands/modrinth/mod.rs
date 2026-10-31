pub mod deps;
pub mod install;
pub mod install_datapack;
pub mod install_hash;
pub mod install_version;
pub mod members;
pub mod project;
pub mod project_by_hash;
pub mod search;
pub mod tags;
pub mod team;
pub mod user;
pub mod user_projects;
pub mod version;
pub mod version_by_hash;
pub mod versions;

use std::path::PathBuf;

use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ModrinthArgs {
    #[arg(
        long = "game-path",
        global = true,
        value_name = "DIR",
        help = "Game directory (used by install subcommands and as a path base)"
    )]
    pub game_path: Option<PathBuf>,

    #[arg(short = 'j', long, global = true, help = "Output raw JSON")]
    pub json: bool,

    #[command(subcommand)]
    pub subcommand: ModrinthCommands,
}

#[derive(Subcommand, Debug)]
pub enum ModrinthCommands {
    Search(search::SearchCliArgs),
    Project(project::ProjectArgs),
    #[command(name = "project-by-hash")]
    ProjectByHash(project_by_hash::ProjectByHashArgs),
    Versions(versions::VersionsArgs),
    Version(version::VersionArgs),
    #[command(name = "version-by-hash")]
    VersionByHash(version_by_hash::VersionByHashArgs),
    Deps(deps::DepsArgs),
    Members(members::MembersArgs),
    User(user::UserArgs),
    #[command(name = "user-projects")]
    UserProjects(user_projects::UserProjectsArgs),
    Team(team::TeamArgs),
    Tags(tags::TagsArgs),
    Install(install::InstallArgs),
    #[command(name = "install-version")]
    InstallVersion(install_version::InstallVersionArgs),
    #[command(name = "install-hash")]
    InstallHash(install_hash::InstallHashArgs),
    #[command(name = "install-datapack")]
    InstallDatapack(install_datapack::InstallDatapackArgs),
}

#[derive(Args, Debug, Default)]
pub struct InstallFlags {
    #[arg(long, value_name = "DIR", help = "Explicit installation directory")]
    pub install_dir: Option<PathBuf>,

    #[arg(long = "mc", value_name = "VERSION", help = "Minecraft version filter")]
    pub minecraft_version: Option<String>,

    #[arg(long, value_name = "LOADER", help = "Mod loader filter")]
    pub loader: Option<String>,

    #[arg(
        long = "version-type",
        value_name = "TYPE",
        help = "release | beta | alpha"
    )]
    pub version_type: Option<String>,

    #[arg(long, help = "Also install required dependencies")]
    pub deps: bool,

    #[arg(long, help = "Overwrite existing files")]
    pub overwrite: bool,
}

impl InstallFlags {
    pub fn to_params(
        &self,
        game_path: Option<PathBuf>,
    ) -> Result<rmlib::extra::modrinth::dto::InstallParams, String> {
        use rmlib::extra::modrinth::dto::VersionType;
        let version_type = match self.version_type.as_deref() {
            None => None,
            Some("release") => Some(VersionType::Release),
            Some("beta") => Some(VersionType::Beta),
            Some("alpha") => Some(VersionType::Alpha),
            Some(other) => {
                return Err(format!(
                    "invalid --version-type {other:?} (expected release, beta or alpha)"
                ));
            }
        };
        Ok(rmlib::extra::modrinth::dto::InstallParams {
            game_path,
            install_dir: self.install_dir.clone(),
            minecraft_version: self.minecraft_version.clone(),
            loader: self.loader.clone(),
            version_type,
            install_required_dependencies: self.deps,
            overwrite: self.overwrite,
        })
    }
}
