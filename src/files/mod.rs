mod config;

use std::{error::Error, fs::File, path::Path};

pub use config::*;

pub fn create_file(path: impl AsRef<Path>) -> File {
    debug!("Creating file on path {:?}", path.as_ref());

    let file = File::create(path).unwrap_or_else(|err| {
        error!("failed to create file: {err:?}");
        std::process::exit(1)
    });

    debug!("Successfully created file");
    trace!("File:{file:?}");
    file
}

pub fn read_on_path(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    debug!("Reading on path {:?}", path.as_ref());

    let bytes = std::fs::read(path)?;
    let contents = String::from_utf8(bytes)?;

    debug!("Successfully read contents on path");
    trace!("Contents read: {contents}");
    Ok(contents)
}

pub fn write_to_path(path: impl AsRef<Path>, contents: &str) -> Result<(), Box<dyn Error>> {
    debug!("Writing on path {:?}", path.as_ref());

    std::fs::write(path, contents)?;

    debug!("Successfully wrote contents to path");
    trace!("Contents written: {contents}");
    Ok(())
}
