use std::{error::Error, path::Path};

use crate::settings::Settings;
use configparser::ini::Ini;
use indexmap::IndexMap;

use super::read_on_path;

static PATH: &str = "C:/Users/harru/Zomboid/Server/servertest.ini";

pub fn try_load_config_map() -> Result<IndexMap<String, String>, Box<dyn Error>> {
    debug!("Checking for config file on path: {PATH}");

    let mut config = create_ini();
    let full_map = config.load(PATH)?;
    debug!("Successfully read config contents");

    let config = full_map.get("default").unwrap().clone();
    let config_map = config
        .into_iter()
        .map(|(k, v)| (k, v.unwrap()))
        .collect::<IndexMap<String, String>>();

    Ok(config_map)
}

pub fn try_load_settings() -> Result<Settings, Box<dyn Error>> {
    let config_map = try_load_config_map()?;
    let settings = Settings::from(config_map);

    debug!("{:?}", settings);
    Ok(settings)
}

pub fn try_save_config_map(map: IndexMap<String, String>) -> Result<(), Box<dyn Error>> {
    let config_str = read_on_path(PATH)?;
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

    debug!("Writing to config");
    std::fs::write(PATH, updated_config_str)?;
    Ok(())
}

/// Makes a case_sensitive Ini object with no ; delimiters
fn create_ini() -> Ini {
    // Make a new case-sensitive Ini object to store the data within
    let mut config = Ini::new_cs();

    // Exclude ; from the delimiters
    config.set_comment_symbols(&['#']);

    config
}
