use std::path::Path;

use anyhow::Context;
use clap::{Args, Subcommand};
use indexmap::IndexMap;

use crate::settings::Settings;

mod get;
mod list;
mod reset;
mod set;
mod validate;

#[derive(Debug, Args, Getters)]
#[command(visible_aliases = ["cfg", "configure"])]
pub struct ConfigArgs {
    /// Path to the Zomboid Server directory
    #[arg(long)]
    directory: Option<String>,

    /// Name of the server ini config file
    #[arg(long)]
    filename: Option<String>,

    #[command(subcommand)]
    command: ConfigCommands,
}

impl ConfigArgs {
    pub fn execute(&self) -> anyhow::Result<()> {
        let dir = self.directory.clone().unwrap_or(crate::fs::server_dir()?);
        let file = self
            .filename
            .clone()
            .unwrap_or(crate::fs::zomboid_config_filename()?);
        let config_path = format!("{dir}/{file}");

        debug!("Path to config file: [{config_path}]");
        self.command.execute(config_path)
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Retrieve a config value from the entry key
    Get(get::GetArgs),
    /// Set the value of a config entry
    Set(set::SetArgs),

    /// List all config entries
    List(list::ListArgs),

    /// Reset all config entries to their default values
    Reset,

    /// Verifies each config entry for valid values
    #[command(hide = true)]
    Validate,
}

impl ConfigCommands {
    pub fn execute(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        use ConfigCommands::*;
        match self {
            Get(args) => args.execute(path),
            Set(args) => args.execute(path),
            List(args) => args.execute(path),
            Reset => reset::cmd(path).context("config reset failed"),
            Validate => validate::cmd(path).context("config validate failed"),
        }
    }
}

fn default_config_map() -> IndexMap<String, String> {
    let settings_defaults = Settings::default();
    let map_defaults: IndexMap<String, String> = settings_defaults.into();
    map_defaults
}
