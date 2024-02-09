use std::path::Path;

use anyhow::Result;

use crate::server::Settings;

/// `spiffo config list --changes`
pub fn list_changes(path: impl AsRef<Path>) -> Result<()> {
    debug!("Printing config map, changed only");

    let settings = Settings::read_from_file(path)?;
    let map_defaults = super::super::default_config_map();

    let mut map_changed = IndexMap::new();
    for (key, default_value) in map_defaults.iter() {
        let current_value = map_current.get(key).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("failed to get map key {key}").as_str()
        ))?;

        if current_value != default_value {
            map_changed.insert(key.clone(), current_value.clone());
        }
    }

    match map_changed.len() {
        l if l == 0 => {
            info!("All config entries are set to the default value.")
        },
        _ => info!(
            "Config entries that are changed from their default
     value:\n{map_changed:#?}"
        )
    }
    Ok(())
}
