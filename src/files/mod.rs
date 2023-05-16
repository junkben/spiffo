mod config;

use std::{error::Error, fs::File, path::Path};

pub use config::*;

pub fn create_file(path: impl AsRef<Path>) -> File {
    let path_ref = path.as_ref();
    File::create(path_ref).unwrap_or_else(|err| {
        error!("failed to create file {path_ref:?}: {err:?}");
        std::process::exit(1)
    })
}

pub fn read_on_path(path: impl AsRef<Path>) -> Result<String, Box<dyn Error>> {
    let bytes = std::fs::read(path)?;
    let content = String::from_utf8(bytes)?;
    Ok(content)
}
