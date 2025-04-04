use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use zip::ZipArchive;

use super::apk_parser::{Item, APK};
use crate::functions::aes_decrypt::aes_decrypt;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::utils::get_folder_name;

impl APK {
    fn get_list_str(
        &mut self,
        zip: &mut ZipArchive<File>,
        item: &str,
    ) -> Result<String, std::io::Error> {
        let mut item_list = zip.by_name(&format!("{}.list", item))?;
        let mut item_list_data = Vec::new();
        item_list.read_to_end(&mut item_list_data)?;

        let result = aes_decrypt::decrypt_ecb(false, &item_list_data.as_slice()).map_err(
            |e: Box<dyn Error>| {
                io::Error::new(io::ErrorKind::Other, format!("Decrypt error: {}", e))
            },
        )?;

        let list_str = String::from_utf8(result).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("UTF-8 decode error: {}", e),
            )
        })?;

        Ok(list_str)
    }

    pub fn parse_item(
        &mut self,
        cc: &str,
        output_path: &str,
        item: &str,
    ) -> Result<(), std::io::Error> {
        log(LogLevel::Info, format!("Start to Parse: {}", item));

        let file = File::open(std::env::temp_dir().join("InstallPack.apk"))?;
        let mut zip = ZipArchive::new(file)?;

        let list_str = match self.get_list_str(&mut zip, item) {
            Ok(s) => s,
            Err(e) => {
                log(LogLevel::Error, format!("Error parsing item list: {}", e));
                return Err(e);
            }
        };

        let mut item_pack = zip.by_name(&format!("{}.pack", item))?;
        let mut item_pack_data = Vec::new();
        item_pack.read_to_end(&mut item_pack_data)?;

        for (i, line) in list_str.lines().enumerate().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let start = match parts[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        log(
                            LogLevel::Warning,
                            format!("Invalid start index at line {}: {}", i + 1, e),
                        );
                        continue;
                    }
                };

                let arrange = match parts[2].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        log(
                            LogLevel::Warning,
                            format!("Invalid arrange size at line {}: {}", i + 1, e),
                        );
                        continue;
                    }
                };

                let item_data = Item {
                    name: parts[0].to_string(),
                    start,
                    arrange,
                };

                let content = &item_pack_data[item_data.start..item_data.start + item_data.arrange];

                let parent_folder = item.rsplit('/').next().unwrap_or("default_folder");
                let output_path = PathBuf::from(output_path)
                    .join(folder_name)
                    .join(get_folder_name(&self.cc))
                    .join("local")
                    .join(parent_folder)
                    .join(&item_data.name);

                item_data.write_file(cc, item, content, output_path)?;
            } else {
                log(
                    LogLevel::Warning,
                    format!("Invalid line format at line {}: {}", i + 1, line),
                );
            }
        }

        Ok(())
    }
}
