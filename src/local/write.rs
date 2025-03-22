use std::path::PathBuf;

use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::{create_dir, create_file};
use crate::local::aes_decrypt::aes_decrypt;
use crate::local::apk_parser::Item;

impl Item {
    pub fn write_file(
        &self,
        cc: &str,
        item: &str,
        content: &[u8],
        fp: PathBuf,
    ) -> Result<(), std::io::Error> {
        let parent_dir = fp
            .parent()
            .and_then(|p| p.to_str())
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path"))?;

        create_dir(parent_dir)?;

        let fp_str = fp.to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file path")
        })?;

        match item {
            "assets/ImageDataLocal" => {
                create_file(content, fp_str)?;
            }
            _ => {
                let data = aes_decrypt::decrypt_pack(cc, content).map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Decrypt error: {:#?}", e),
                    )
                })?;
                create_file(&data, fp_str)?;
            }
        }
        log(
            LogLevel::Info,
            format!(
                "Success Write File {}",
                fp.file_name().unwrap_or_default().to_string_lossy()
            ),
        );

        Ok(())
    }
}
