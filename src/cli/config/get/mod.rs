use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};

mod entry;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct GetArgs {
    #[command(subcommand)]
    command: Option<GetCommands>,

    #[command(flatten)]
    key_pair: entry::GetEntryArgs,
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
    Entry(entry::GetEntryArgs),
}

impl GetCommands {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        use GetCommands::*;
        match self {
            Entry(args) => args.execute(path),
        }
    }
}
