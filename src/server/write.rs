use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;

use crate::functions::aes_decrypt::aes_decrypt;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::{create_dir, create_file};

use super::parse_zip::Item;

impl Item {
    pub fn write_file(
        &self,
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

        if fp_str.contains("Audio") {
            create_file(content, fp_str)?;
        } else {
            match item {
                "ImageDataLocal" => {
                    create_file(content, fp_str)?;
                }
                _ => {
                    let mut data = aes_decrypt::decrypt_ecb(true, content).map_err(|e| {
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Decrypt error: {:#?}", e),
                        )
                    })?;

                    if fp.extension().and_then(OsStr::to_str) == Some("json") {
                        let json_str = String::from_utf8(data).map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("UTF-8 decode error: {}", e),
                            )
                        })?;

                        let json: serde_json::Value =
                            serde_json::from_str(&json_str).map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("JSON parse error: {}", e),
                                )
                            })?;

                        data = serde_json::to_string_pretty(&json)
                            .map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!("JSON serialize error: {}", e),
                                )
                            })?
                            .into_bytes();
                    }

                    create_file(&data, fp_str)?;
                }
            }
        }

        log(
            LogLevel::Info,
            format!(
                "Successfully wrote file: {}",
                fp.file_name().unwrap_or_default().to_string_lossy()
            ),
        );

        Ok(())
    }
}
