mod config;

use anyhow::{Context, Result};
use std::{fs::File, path::Path};

pub use config::*;

pub fn create_file(path: impl AsRef<Path>) -> Result<File> {
    debug!("Creating file on path {:?}", path.as_ref());

    let file = File::create(path).context("failed to create file")?;

    debug!("Successfully created file");
    trace!("File:{file:?}");
    Ok(file)
}

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
