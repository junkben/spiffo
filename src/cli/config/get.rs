use anyhow::Result;
use std::path::Path;

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
