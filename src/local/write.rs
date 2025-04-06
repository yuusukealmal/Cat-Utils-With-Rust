use crate::config::structs::LocalItem;
use crate::functions::json_prettier::indent_json;
use crate::functions::writer::writer::{create_dir, create_file};
use std::ffi::OsStr;
use std::io;

impl LocalItem {
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

        create_file(&content, fp_str)?;

        Ok(())
    }
}
