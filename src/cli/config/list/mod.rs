use std::path::Path;

use anyhow::Result;
use clap::Args;

mod changes;
mod search;
mod subset;

pub use subset::ListSubsets;

#[derive(Debug, Args, Getters)]
pub struct ListArgs {
    /// Only list config values that are changed from their defaults
    #[arg(short, long, default_value_t = false)]
    changes: bool,

    /// List only a subset of settings
    #[arg(long)]
    subset: Option<ListSubsets>,

    /// Searches for config entries by key
    #[arg(long)]
    search: Option<String>,
}

impl ListArgs {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        if let Some(subset) = &self.subset {
            return subset.execute(path);
        }
        if let Some(find) = &self.search {
            return search::list_search(path, find);
        }
        match self.changes {
            true => changes::list_changes(path),
            false => list(path),
        }
    }
}

/// `spiffo config list`
pub fn list(path: impl AsRef<Path>) -> Result<()> {
    debug!("Printing config map");

    let config_map = crate::fs::read_config_map(path)?;

    info!("Config Settings Map:\n{:#?}", config_map);
    Ok(())
}