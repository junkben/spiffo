use indexmap::IndexMap;

use crate::settings::Settings;

/// `spiffo config reset`
pub fn cmd() {
    debug!("Resetting config map to defaults");

    let settings_defaults = Settings::default();
    let map_defaults: IndexMap<String, String> = settings_defaults.into();
    todo!()
}
