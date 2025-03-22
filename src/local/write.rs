use std::path::PathBuf;

use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::{create_dir, create_file};
use crate::local::aes_decrypt::aes_decrypt;
use crate::local::apk_parser::Item;

impl Item {
    pub fn write_file(
        &self,
        item: &str,
        key: &str,
        iv: &str,
        content: &[u8],
        fp: PathBuf,
    ) -> Result<(), std::io::Error> {
        let _ = create_dir(fp.parent().unwrap().to_str().unwrap())?;
        if item == "assets/ImageDataLocal" {
            let _ = create_file(content, fp.to_str().unwrap())?;
        } else {
            let data = aes_decrypt::decrypt_pack(key, iv, content).unwrap();
            let _ = create_file(&data, fp.to_str().unwrap())?;
        }
        log(
            LogLevel::Info,
            format!(
                "Success Write File {}",
                fp.file_name().unwrap().to_str().unwrap()
            ),
        );
        Ok(())
    }
}
