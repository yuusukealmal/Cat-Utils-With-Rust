use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use zip::ZipArchive;

use crate::functions::logger::logger::{log, LogLevel};
use crate::local::aes_decrypt::aes_decrypt;
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

        let result = aes_decrypt::decrypt_list(
            &self.list_key.as_bytes().to_vec(),
            &item_list_data.as_slice(),
        );

        let list_str = String::from_utf8(result.unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(list_str)
    }

    pub fn parse_item(&mut self, output_path: &str, item: &str) -> Result<(), std::io::Error> {
        log(LogLevel::Info, format!("Parsing item: {}", item));

        let file = File::open("InstallPack.apk")?;
        let mut zip = ZipArchive::new(file)?;

        let list = self.get_list_str(&mut zip, item);

        match list {
            Ok(list_str) => {
                let mut item_pack = zip.by_name(&format!("{}.pack", item))?;
                let mut item_pack_data = Vec::new();
                item_pack.read_to_end(&mut item_pack_data)?;

                for line in list_str.lines().skip(1) {
                    if line.split(",").count() == 3 {
                        let parts: Vec<&str> = line.split(',').collect();
                        let file = Item {
                            name: parts[0].to_string(),
                            start: parts[1].parse::<usize>().unwrap(),
                            arrange: parts[2].parse::<usize>().unwrap(),
                        };

                        let content = &item_pack_data[file.start..file.start + file.arrange];

                        let package = format!("jp.co.ponos.battlecats.{}", self.cc);
                        let folder = item.split("/").last().unwrap();
                        let output_path = PathBuf::from(output_path)
                            .join(package)
                            .join(folder)
                            .join(&file.name);

                        let _ = file.write_file(
                            item,
                            &self.pack_key,
                            &self.pack_iv,
                            content,
                            output_path,
                        );
                    }
                }
            }
            Err(e) => {
                log(LogLevel::Error, format!("Error parsing item: {}", e));
            }
        }

        Ok(())
    }
}
