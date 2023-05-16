use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SetArgs {
    #[command(subcommand)]
    command: Option<SetCommands>,

    #[command(flatten)]
    key_pair: SetEntryArgs,
}

impl SetArgs {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        let binding = SetCommands::Entry(self.key_pair.clone());
        let set_cmd = match &self.command {
            Some(s) => s,
            None => &binding,
        };
        set_cmd.execute(path)
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum SetCommands {
    /// Set the value of a config entry
    Entry(SetEntryArgs),
    /// Mass entry of config file with content verbatim. Does not validate new entries.
    Dump {
        /// Exact data to write to config file
        contents: String,
    },
}

impl SetCommands {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        use SetCommands::*;
        match self {
            Entry(args) => args.execute(path),
            Dump { contents } => crate::fs::write_to_path(path, contents),
        }
    }
}

#[derive(Debug, Args, Getters, Clone)]
pub struct SetEntryArgs {
    /// The key of the config entry
    key: String,
    /// The new value to write to the config entry
    value: String,

    /// Force writing the key-value pair entry without regard for validation
    #[arg(short, long)]
    force: bool,
}

impl SetEntryArgs {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        cmd(self.key(), self.value(), path)
    }
}

/// `spiffo config set <KEY> <VALUE>`
pub fn cmd(key: &str, value: &str, path: impl AsRef<Path>) -> Result<()> {
    debug!("Changing setting {key} => {value}");

    let mut config_map = crate::fs::read_config_map(path.as_ref())?;

    if !config_map.contains_key(key) {
        info!("{key} not found in config");
        return Ok(());
    }

    // This unwrap should never result in a panic
    let old_value = config_map
        .insert(key.to_string(), value.to_string())
        .unwrap();

    if old_value == value {
        info!("{key} is already set to: {old_value}");
        Ok(())
    } else {
        info!("{key}: {old_value} => {value}");
        crate::fs::write_to_config_map(config_map, path)
    }
}
