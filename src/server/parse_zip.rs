use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use zip::ZipArchive;

use crate::functions::{
    aes_decrypt::aes_decrypt,
    logger::logger::{log, LogLevel},
};

pub struct Item {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
}

pub fn parse_zip(cc: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(std::env::temp_dir().join("temp.zip"))?;
    let mut zip = ZipArchive::new(file)?;

    let item_names: Vec<String> = zip
        .file_names()
        .filter(|name| name.contains(".list"))
        .map(|name| name.replace(".list", ""))
        .collect();

    for item_name in item_names {
        let item_list_data = {
            let mut item_list = zip.by_name(&format!("{}.list", item_name))?;
            let mut buf = Vec::new();
            item_list.read_to_end(&mut buf)?;
            buf
        };

        let item_pack_data = {
            let mut item_pack = zip.by_name(&format!("{}.pack", item_name))?;
            let mut buf = Vec::new();
            item_pack.read_to_end(&mut buf)?;
            buf
        };

        let result = aes_decrypt::decrypt_ecb(false, &item_list_data.as_slice())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Decrypt error: {}", e)))?;

        let list_str = String::from_utf8(result).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("UTF-8 decode error: {}", e),
            )
        })?;

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

                let package = format!("jp.co.ponos.battlecats.{} Server", cc);

                let output_path = if file.name.contains(".ogg") || file.name.contains(".caf") {
                    PathBuf::from(output_path)
                        .join(package)
                        .join("Audio")
                        .join(&file.name)
                } else {
                    PathBuf::from(output_path)
                        .join(package)
                        .join(item_name.clone())
                        .join(&file.name)
                };

                file.write_file(&item_name, content, output_path)?;
            } else {
                log(
                    LogLevel::Warning,
                    format!("Invalid line format at line {}: {}", i + 1, line),
                );
            }
        }
    }

    Ok(())
}
