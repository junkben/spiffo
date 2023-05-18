use std::path::Path;

use anyhow::Result;
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct GetEntryArgs {
    /// The key of the config entry
    key: String,

    /// Force writing the key-value pair entry without regard for validation
    #[arg(short, long)]
    force: bool,
}

impl GetEntryArgs {
    /// `spiffo config get <KEY>`
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        let key = self.key.as_str();
        debug!("Printing config value for {key}");

        let config_map = crate::fs::read_config_map(path)?;
        match config_map.get(key) {
            Some(value) => info!("{key}={value}"),
            None => info!("No config entry found for {key}."),
        }
        Ok(())
    }
}
