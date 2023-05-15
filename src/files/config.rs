use std::error::Error;

use crate::settings::Settings;
use configparser::ini::Ini;
use indexmap::IndexMap;

pub fn load_config_map() -> Result<IndexMap<String, String>, Box<dyn Error>> {
    // Make a new case-sensitive Ini object to store the data within
    let mut config = Ini::new_cs();

    // Exclude ; from the delimiters
    config.set_comment_symbols(&['#']);

    let path = "C:/Users/harru/Zomboid/Server/servertest.ini";
    debug!("Checking for config file on path: {path}");

    let full_map = config.load(path)?;
    debug!("Loaded config contents");

    let config = full_map.get("default").unwrap().clone();
    let config_map = config
        .into_iter()
        .map(|(k, v)| (k, v.unwrap()))
        .collect::<IndexMap<String, String>>();

    Ok(config_map)
}

pub fn load_settings() -> Result<Settings, Box<dyn Error>> {
    let config_map = load_config_map()?;
    let settings = Settings::from(config_map);

    debug!("{:?}", settings);
    Ok(settings)
}

pub fn save_config_map(map: IndexMap<String, String>) -> Result<(), Box<dyn Error>> {
    todo!()
}
