use std::path::Path;

use anyhow::Result;
use indexmap::IndexMap;

use crate::{fs::ReadFromFile, server::Settings};

/// `spiffo config list --search <SEARCH>`
pub fn list_search(path: impl AsRef<Path>, find: &str) -> Result<()> {
    debug!("Listing config entries including {find}");

    let pat = find.to_ascii_lowercase();
    let settings = Settings::read_from_file(path)?;
    let settings_value = serde_json::to_value(settings)?;
    let settings_map = settings_value.as_object().unwrap();
    let matching_entries = settings_map
        .into_iter()
        .filter(|&(k, _)| (*k).as_str().to_ascii_lowercase().contains(pat.as_str()))
        .map(|(k, v)| (k.as_str(), v.as_str().unwrap()))
        .collect::<IndexMap<&str, &str>>();

    match matching_entries.len() {
        0 => info!("No config entries matching \"{find}\"."),
        _ => info!("{:#?}", matching_entries)
    }

    Ok(())
}
