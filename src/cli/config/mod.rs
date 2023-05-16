use clap::{Args, Subcommand};
use indexmap::IndexMap;

use crate::settings::Settings;

mod get;
mod list;
mod reset;
mod set;

#[derive(Debug, Args, Getters)]
pub struct ConfigArgs {
    /// Absolute path to the Zomboid Server directory
    #[arg(short, long, env = "SERVER_DIR", default_value_t = format!("~/Zomboid/Server"))]
    directory: String,

    /// Name of the server ini config file
    #[arg(short, long, env = "SERVER_FILENAME", default_value_t = format!("servertest.ini"))]
    filename: String,

    #[command(subcommand)]
    command: ConfigCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Retrieve a config value from the entry key
    Get {
        /// The key of the config entry
        key: String,
    },
    /// Set the value of a config entry
    Set {
        /// The key of the config entry
        key: String,
        /// The new value to write to the config entry
        value: String,
    },

    /// Reset all config entries to their default values
    Reset,

    /// List all config entries
    List(list::ListArgs),
}

impl ConfigCommands {
    pub fn execute(&self) {
        use ConfigCommands::*;
        match self {
            Get { key } => get::cmd(key),
            Set { key, value } => set::cmd(key, value),
            Reset => reset::cmd(),
            List(args) => args.execute(),
        }
    }
}

fn load_config_map() -> IndexMap<String, String> {
    crate::files::try_load_config_map().expect("failed to load config map")
}

/// Sets the values of config keys via IndexMap
fn save_config_map(map: IndexMap<String, String>) {
    crate::files::try_save_config_map(map).expect("failed to save config map")
}

fn default_config_map() -> IndexMap<String, String> {
    let settings_defaults = Settings::default();
    let map_defaults: IndexMap<String, String> = settings_defaults.into();
    map_defaults
}
