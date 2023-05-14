use std::error::Error;

use configparser::ini::Ini;
use indexmap::IndexMap;
use crate::cli::Settings;

pub fn load_config() -> Result<Settings, Box<dyn Error>> {
    // Make a new case-sensitive Ini object to store the data within
    let mut config = Ini::new_cs();

    // Exclude ; from the delimiters
    config.set_comment_symbols(&['#']);

    let path = "C:/Users/harru/Zomboid/Server/servertest.ini";
    debug!("Checking for config file on path: {path}");

    let full_map = config.load(path).unwrap();
    debug!("Loaded config contents");

    let config = full_map.get("default").unwrap().clone();
    let config_no_options = config.into_iter()
        .map(|(k, v)| (k, v.unwrap()))
        .collect::<IndexMap<String, String>>();
    
    let config_string = format!("{:?}", &config_no_options);
    let settings: Settings = serde_json::from_str(&config_string).unwrap();

    debug!("{:?}", settings);
    Ok(settings)
}