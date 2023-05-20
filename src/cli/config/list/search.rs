use std::path::Path;

use anyhow::Result;
use indexmap::IndexMap;

/// `spiffo config list --search <SEARCH>`
pub fn list_search(path: impl AsRef<Path>, find: &str) -> Result<()> {
    debug!("Listing config entries including {find}");

    let config_map = crate::fs::read_config_map(path)?;
    let matching_entries: IndexMap<String, String> = config_map
        .into_iter()
        .filter(|(key, _)| {
            key.to_ascii_lowercase()
                .contains(&find.to_ascii_lowercase())
        })
        .collect::<IndexMap<String, String>>();

    match matching_entries.len() {
        0 => info!("No config entries matching \"{find}\"."),
        _ => info!("Config entries matching \"{find}\":\n{matching_entries:#?}"),
    }
    Ok(())
}
