use std::path::Path;

use crate::settings::Settings;
use anyhow::{Context, Result};
use configparser::ini::Ini;
use indexmap::IndexMap;

pub fn read_config_map(path: impl AsRef<Path>) -> Result<IndexMap<String, String>> {
    debug!("Checking for config file on path: {:?}", path.as_ref());

    let mut config = create_ini();
    let full_map = config.load(path).expect("config load failed");

    let config = full_map
        .get("default")
        .expect("no \"default\" section")
        .clone();

    let config_map = config
        .into_iter()
        .map(|(k, v)| (k, v.unwrap()))
        .collect::<IndexMap<String, String>>();

    Ok(config_map)
}

pub fn read_settings_from_config(path: impl AsRef<Path>) -> Result<Settings> {
    let config_map = read_config_map(path).context("failed to read config map")?;
    let settings = Settings::from(config_map);

    debug!("{:#?}", settings);
    Ok(settings)
}

pub fn write_to_config_map(map: IndexMap<String, String>, path: impl AsRef<Path>) -> Result<()> {
    let config_str = crate::fs::read_on_path(path.as_ref())?;
    let updated_config_str = config_str
        .split_terminator("\r\n")
        .map(|entry| {
            let (lhs, _) = match entry.split_once("=") {
                Some((l, r)) => (l, r),
                None => return entry.to_string(),
            };

            let new_value = match map.keys().find(|&k| entry.contains(&format!("{k}="))) {
                Some(key) => map.get(key).unwrap(),
                None => return entry.to_string(),
            };

            [lhs, new_value].join("=")
        })
        .collect::<Vec<_>>()
        .join("\r\n");

    super::write_to_path(path, &updated_config_str)
}

/// Makes a case_sensitive Ini object with no ; delimiters
fn create_ini() -> Ini {
    // Make a new case-sensitive Ini object to store the data within
    let mut config = Ini::new_cs();

    // Exclude ; from the delimiters
    config.set_comment_symbols(&['#']);

    config
}
