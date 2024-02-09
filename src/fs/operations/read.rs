use std::fs;

use serde::de::DeserializeOwned;

pub trait ReadFromFile: DeserializeOwned {
    fn read_from_file(file_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        debug!("Reading on path {:?}", &file_path.as_ref());

        let mut bytes: Vec<u8> = fs::read(&file_path)?;
        let contents: String = String::from_utf8(bytes).unwrap();
        let object: Self = serde_json::from_str(contents.as_str()).unwrap();

        debug!("Successfully read contents on path");
        trace!("Contents read: {contents}");
        Ok(object)
    }
}
