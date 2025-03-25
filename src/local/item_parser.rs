use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use zip::ZipArchive;

use crate::functions::aes_decrypt::aes_decrypt;
use crate::functions::logger::logger::{log, LogLevel};
use crate::local::apk_parser::{Item, APK};

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
        log(LogLevel::Info, format!("Parsing item: {}", item));

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
                let start = parts[1]
                    .parse::<usize>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let arrange = parts[2]
                    .parse::<usize>()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                let file = Item {
                    name: parts[0].to_string(),
                    start,
                    arrange,
                };

                let content = &item_pack_data[file.start..file.start + file.arrange];

                let package = format!("jp.co.ponos.battlecats.{} Local", cc);
                let folder = item.split('/').last().unwrap();
                let output_path = PathBuf::from(output_path)
                    .join(package)
                    .join(folder)
                    .join(&file.name);

                file.write_file(cc, item, content, output_path)?;
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
