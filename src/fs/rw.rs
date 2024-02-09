use std::path::Path;

use anyhow::{Context, Result};

pub fn read_on_path(path: impl AsRef<Path>) -> Result<String> {
    debug!("Reading on path {:?}", path.as_ref());

    let bytes = std::fs::read(path).context("failed filesystem read")?;
    let contents = String::from_utf8(bytes).context("file contains invalid String char")?;

    debug!("Successfully read contents on path");
    trace!("Contents read: {contents}");
    Ok(contents)
}

pub fn write_to_path(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    debug!("Writing on path {:?}", path.as_ref());

    std::fs::write(path, contents).context("failed filesystem write")?;

    debug!("Successfully wrote contents to path");
    trace!("Contents written: {contents}");
    Ok(())
}
