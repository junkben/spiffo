use std::path::Path;

/// `spiffo config get <KEY>`
pub fn cmd(key: &str, path: impl AsRef<Path>) {
    debug!("Printing config value for {key}");
    match get_config_value(key, path) {
        Some(value) => info!("{key}={value}"),
        None => info!("No config entry found for {key}."),
    }
}

fn get_config_value(key: &str, path: impl AsRef<Path>) -> Option<String> {
    super::read_config_map(path).get(key).cloned()
}
