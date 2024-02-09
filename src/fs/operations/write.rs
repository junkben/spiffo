use std::fs;

use anyhow::Context;
use serde::Serialize;

pub trait WriteToFile: Sized + Serialize {
    fn write_to_path(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(self)?;

        debug!("Writing on path {:?}", &path.as_ref());

        fs::write(&path, contents).context("failed filesystem write")?;

        debug!("Successfully wrote contents to path");
        Ok(())
    }
}
