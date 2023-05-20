use std::path::Path;

use crate::java::JavaArgs;
use anyhow::Result;

pub fn read_java_config() -> Result<JavaArgs> {
    debug!("Reading java config file on default path");
    read_java_config_on_path(super::java_config_path()?)
}

pub fn read_java_config_on_path(path: impl AsRef<Path>) -> Result<JavaArgs> {
    debug!("Reading java config file on path: {:?}", path.as_ref());

    let java_config_str = super::read_on_path(path.as_ref())?;
    let java_config: JavaArgs = serde_json::from_str(java_config_str.clone().as_str())?;
    Ok(java_config)
}

pub fn write_to_java_config(java_config: JavaArgs) -> Result<()> {
    debug!("Writing to java config file on default path");
    write_to_java_config_on_path(java_config, super::java_config_path()?)
}

pub fn write_to_java_config_on_path(java_config: JavaArgs, path: impl AsRef<Path>) -> Result<()> {
    debug!("Writing to java config file on path: {:?}", path.as_ref());

    let contents = serde_json::to_string_pretty(&java_config)?;
    super::write_to_path(path, &contents)
}
