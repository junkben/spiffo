/// `spiffo config get <KEY>`
pub fn cmd(key: &str) {
    debug!("Printing config value for {key}");
    match get_config_value(key) {
        Some(value) => info!("{key}={value}"),
        None => info!("No value found for {key}."),
    }
}

fn get_config_value(key: &str) -> Option<String> {
    super::load_config_map().get(key).cloned()
}
