use std::path::Path;

/// `spiffo config reset`
pub fn cmd(path: impl AsRef<Path>) {
    debug!("Resetting config map to defaults");

    let mut map = super::read_config_map(path.as_ref());
    let map_defaults = super::default_config_map();

    for (key, default_value) in map_defaults.iter() {
        map.insert(key.clone(), default_value.clone());
    }

    super::write_to_config_map(map, path)
}
