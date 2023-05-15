use indexmap::IndexMap;

/// `spiffo config set <KEY> <VALUE>`
pub fn cmd(key: &str, value: &str) {
    debug!("Changing setting {key} => {value}");

    todo!()
}

/// Sets the values of config keys via IndexMap
fn set_config_map(map: &IndexMap<String, String>) {
    todo!()
}
