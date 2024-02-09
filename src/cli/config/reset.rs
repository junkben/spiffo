use std::path::Path;

use crate::{fs::WriteToFile, server::Settings};

/// `spiffo config reset`
pub fn cmd(path: impl AsRef<Path>) -> anyhow::Result<()> {
    debug!("Resetting config map to defaults");

    let default_settings = Settings::default();
    default_settings.write_to_path(path)
}
