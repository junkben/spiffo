use indexmap::IndexMap;

use crate::settings::Settings;

/// `spiffo config reset`
pub fn cmd() {
    debug!("Resetting config map to defaults");

    let mut map = super::load_config_map();
    let map_defaults = super::default_config_map();

    for (key, default_value) in map_defaults.iter() {
        map.insert(key.clone(), default_value.clone());
    }

    super::save_config_map(map)
}
