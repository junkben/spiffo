use std::path::Path;

use clap::{Args, Subcommand};
use indexmap::IndexMap;

use crate::settings::Settings;

mod get;
mod list;
mod reset;
mod set;
mod validate;

#[derive(Debug, Args, Getters)]
pub struct ConfigArgs {
    /// Home path, typically where the server directory lives
    #[arg(long, env = "HOME")]
    home: String,

    /// Path to the Zomboid Server directory, assuming it's somewhere within $HOME
    #[arg(short, long, env = "SERVER_DIR", default_value_t = format!("Zomboid/Server"))]
    directory: String,

    /// Name of the server ini config file
    #[arg(short, long, env = "SERVER_FILENAME", default_value_t = format!("servertest.ini"))]
    filename: String,

    #[command(subcommand)]
    command: ConfigCommands,
}

impl ConfigArgs {
    pub fn execute(&self) {
        let config_path = format!("{}/{}/{}", self.home, self.directory, self.filename);
        
        debug!("Path to config file: [{config_path}]");
        self.command.execute(config_path)
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Retrieve a config value from the entry key
    Get {
        /// The key of the config entry
        key: String,
    },
    /// Set the value of a config entry
    Set(set::SetArgs),

    /// List all config entries
    List(list::ListArgs),

    /// Reset all config entries to their default values
    Reset,

    /// Verifies each config entry for valid values
    Validate,
}

impl ConfigCommands {
    pub fn execute(&self, path: impl AsRef<Path>) {
        use ConfigCommands::*;
        match self {
            Get { key } => get::cmd(key, path),
            Set(args) => args.execute(path),
            List(args) => args.execute(path),
            Reset => reset::cmd(path),
            Validate => validate::cmd(path),
        }
    }
}

fn read_config_map(path: impl AsRef<Path>) -> IndexMap<String, String> {
    crate::files::read_config_map(path).expect("failed to load config map")
}

/// Sets the values of config keys via IndexMap
fn write_to_config_map(map: IndexMap<String, String>, path: impl AsRef<Path>) {
    crate::files::write_to_config_map(map, path).expect("failed to save config map")
}

fn default_config_map() -> IndexMap<String, String> {
    let settings_defaults = Settings::default();
    let map_defaults: IndexMap<String, String> = settings_defaults.into();
    map_defaults
}
