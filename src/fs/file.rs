// use std::{fs::File, path::Path};
//
// use anyhow::{Context, Result};
//
// pub fn create_file(path: impl AsRef<Path>) -> Result<File> {
//     debug!("Creating file on path {:?}", path.as_ref());
//
//     let file = File::create(path).context("failed to create file")?;
//
//     debug!("Successfully created file");
//     trace!("File:{file:?}");
//     Ok(file)
// }
