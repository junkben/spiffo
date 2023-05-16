use indexmap::IndexMap;

/// `spiffo config set <KEY> <VALUE>`
pub fn cmd(key: &str, value: &str) {
    debug!("Changing setting {key} => {value}");

    set_config_map(key.to_string(), value.to_string())
}

/// Sets the values of config keys via IndexMap
fn set_config_map(key: String, value: String) {
    let mut config_map = super::load_config_map();

    let old_value = config_map
        .insert(key.clone(), value.clone())
        .unwrap_or_else(|| {
            error!("key {key} does not exist");
            std::process::exit(1)
        });

    if old_value == value {
        info!("{key} is already set to: {old_value}")
    } else {
        info!("{key}: {old_value} => {value}");
        super::save_config_map(config_map)
    }
}
