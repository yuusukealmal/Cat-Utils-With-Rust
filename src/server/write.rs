use std::ffi::OsStr;
use std::{fs, io};

use crate::config::structs::ServerItem;
use crate::functions::json_prettier::indent_json;
use crate::functions::md5_check::get_hash;
use crate::functions::writer::writer::{create_dir, create_file};

impl ServerItem {
    pub fn write_file(&self, mut content: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let parent_dir = self
            .output_path
            .parent()
            .and_then(|p| p.to_str())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))?;

        create_dir(parent_dir)?;

        let fp_str = self.output_path.to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file path")
        })?;

        let existing_hash = if fs::metadata(fp_str).is_ok() {
            get_hash(fp_str)?
        } else {
            String::new()
        };

        if self.output_path.extension().and_then(OsStr::to_str) == Some("json")
            && !content.is_empty()
        {
            let mut json: serde_json::Map<String, serde_json::Value> =
                serde_json::from_slice(&content).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("JSON parse error: {}", e),
                    )
                })?;

            content = indent_json(&mut json)?.into();
        }

        let current_hash = get_hash(&content)?;

        if current_hash != existing_hash {
            create_file(&content, fp_str)?;
        }

        Ok(())
    }
}
