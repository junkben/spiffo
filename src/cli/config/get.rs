use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct GetArgs {
    #[command(subcommand)]
    command: Option<GetCommands>,

    #[command(flatten)]
    key_pair: GetEntryArgs,
}

impl GetArgs {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        let binding = GetCommands::Entry(self.key_pair.clone());
        let get_cmd = match &self.command {
            Some(s) => s,
            None => &binding,
        };
        get_cmd.execute(path)
    }
}

#[derive(Debug, Subcommand, Clone)]
pub enum GetCommands {
    /// Get the value of a config entry
    Entry(GetEntryArgs),

    /// Mass entry of config file with content verbatim. Does not validate new entries.
    Dump {
        /// Exact data to write to config file
        contents: String,
    },
}

impl GetCommands {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        use GetCommands::*;
        match self {
            Entry(args) => args.execute(path),
            Dump { contents } => crate::fs::write_to_path(path, contents),
        }
    }
}

#[derive(Debug, Args, Getters, Clone)]
pub struct GetEntryArgs {
    /// The key of the config entry
    key: String,
    /// The new value to write to the config entry
    value: String,

    /// Force writing the key-value pair entry without regard for validation
    #[arg(short, long)]
    force: bool,
}

impl GetEntryArgs {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        cmd(self.key(), path)
    }
}

/// `spiffo config get <KEY>`
pub fn cmd(key: &str, path: impl AsRef<Path>) -> Result<()> {
    debug!("Printing config value for {key}");

    let config_map = crate::fs::read_config_map(path)?;
    match config_map.get(key) {
        Some(value) => info!("{key}={value}"),
        None => info!("No config entry found for {key}."),
    }
    Ok(())
}
