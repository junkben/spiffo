use std::path::Path;

use anyhow::Result;

/// `spiffo config reset`
pub fn cmd(_path: impl AsRef<Path>) -> Result<()> {
    debug!("Validating config map entries");

    error!("`config validate` not yet implemented");

    Ok(())
}
