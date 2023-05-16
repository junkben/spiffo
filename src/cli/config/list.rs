use std::path::Path;

use clap::Args;
use indexmap::IndexMap;

#[derive(Debug, Args, Getters)]
pub struct ListArgs {
    /// Only list config values that are changed from their defaults
    #[arg(short, long)]
    changes: bool,
}

impl ListArgs {
    pub fn execute(&self, path: impl AsRef<Path>) {
        match self.changes {
            true => list_changes(path),
            false => list(path),
        }
    }
}

/// `spiffo config list`
pub fn list(path: impl AsRef<Path>) {
    debug!("Printing config map");

    let config_map = super::read_config_map(path);
    info!("Config Settings Map:\n{:#?}", config_map)
}

/// `spiffo config list --changes`
pub fn list_changes(path: impl AsRef<Path>) {
    debug!("Printing config map, changed only");

    let map_current = super::read_config_map(path);
    let map_defaults = super::default_config_map();

    let mut map_changed = IndexMap::new();
    for (key, default_value) in map_defaults.iter() {
        let current_value = map_current.get(key).unwrap_or_else(|| {
            error!("failed to find key {key} in map_defaults");
            std::process::exit(1)
        });

        if current_value != default_value {
            map_changed.insert(key.clone(), current_value.clone());
        }
    }

    match map_changed.len() {
        l if l == 0 => info!("All config entries are set to the default value."),
        _ => info!("Config entries that are changed from their default:\n{map_changed:#?}"),
    }
}
