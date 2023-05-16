use std::path::Path;

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
    pub fn execute(&self, path: impl AsRef<Path>) {
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
        contents: String
    }
}

impl SetCommands {
    pub fn execute(&self, path: impl AsRef<Path>) {
        use SetCommands::*;
        match self {
            Entry(args) => args.execute(path),
            Dump { contents } => crate::files::write_to_path(path, contents).unwrap(),
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
    force: bool
}

impl SetEntryArgs {
    pub fn execute(&self, path: impl AsRef<Path>) {
        cmd(self.key(), self.value(), path)
    }
}

/// `spiffo config set <KEY> <VALUE>`
pub fn cmd(key: &str, value: &str, path: impl AsRef<Path>) {
    debug!("Changing setting {key} => {value}");

    set_config_map(key.to_string(), value.to_string(), path)
}

/// Sets the values of config keys via IndexMap
fn set_config_map(key: String, value: String, path: impl AsRef<Path>) {
    let mut config_map = super::read_config_map(path.as_ref());

    let old_value = config_map
        .insert(key.clone(), value.clone())
        .unwrap_or_else(|| {
            error!("key {key} does not exist");
            std::process::exit(1)
        });

    if old_value == value {
        info!("{key} is already set to: {old_value}")
    } else {
        info!("{key}: {old_value} => {value}");
        super::write_to_config_map(config_map, path)
    }
}
