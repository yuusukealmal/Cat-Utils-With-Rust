use std::error::Error;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::config::structs::{APK, LocalItem};
use crate::functions::aes_decrypt::aes_decrypt;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::utils::get_folder_name;

impl APK {
    fn get_list_str(&mut self, item: &str) -> Result<String, std::io::Error> {
        let mut item_list = self.zip.by_name(&format!("{}.list", item))?;
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

    pub fn parse_item(&mut self, item: &str) -> Result<(), Box<dyn std::error::Error>> {
        log(LogLevel::Info, format!("Start to Parse: {}", item));

        let list_str = match self.get_list_str(item) {
            Ok(s) => s,
            Err(e) => {
                log(LogLevel::Error, format!("Error parsing item list: {}", e));
                return Err(e.into());
            }
        };

        let mut item_pack = self.zip.by_name(&format!("{}.pack", item))?;
        let mut item_pack_data = Vec::new();
        item_pack.read_to_end(&mut item_pack_data)?;

        for (index, line) in list_str.lines().enumerate().skip(1) {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let name = parts[0].to_string();
                let start = match parts[1].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        log(
                            LogLevel::Error,
                            format!("Invalid start index at line {}: {}", index + 1, e),
                        );
                        continue;
                    }
                };

                let arrange = match parts[2].parse::<usize>() {
                    Ok(v) => v,
                    Err(e) => {
                        log(
                            LogLevel::Error,
                            format!("Invalid arrange size at line {}: {}", index + 1, e),
                        );
                        continue;
                    }
                };

                let output_path = PathBuf::from(&self.output_path)
                    .join(get_folder_name(&self.cc))
                    .join("local")
                    .join(item.rsplit("/").next().unwrap())
                    .join(&name);

                let content = LocalItem {
                    name,
                    start,
                    arrange,
                    output_path,
                };

                let data = &item_pack_data[content.start..content.start + content.arrange];

                content.write_file(&self.cc, item, data)?;
            } else {
                log(
                    LogLevel::Error,
                    format!("Invalid line format at line {}: {}", index + 1, line),
                );
            }
        }

        Ok(())
    }
}
