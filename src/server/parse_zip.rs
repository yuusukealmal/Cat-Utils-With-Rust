use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

use zip::ZipArchive;

use crate::functions::aes_decrypt::aes_decrypt;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::utils::get_folder_name;
use crate::functions::writer::{create_dir, create_file};

pub struct Item {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
}

impl Item {
    fn from_line(line: &str) -> Result<Self, io::Error> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let start = parts[1].parse::<usize>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse start: {}", e),
                )
            })?;
            let arrange = parts[2].parse::<usize>().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to parse arrange: {}", e),
                )
            })?;

            Ok(Item {
                name: parts[0].to_string(),
                start,
                arrange,
            })
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid line format",
            ))
        }
    }
}

fn read_file_from_zip(zip: &mut ZipArchive<File>, file_name: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = zip.by_name(file_name)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn parse_zip(cc: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(std::env::temp_dir().join("temp.zip"))?;
    let mut zip = ZipArchive::new(file)?;

    let item_names: Vec<String> = zip
        .file_names()
        .filter(|name| name.contains(".list") || name.contains(".ogg") || name.contains(".caf"))
        .map(|name| name.replace(".list", ""))
        .collect();

    for item_name in item_names {
        log(LogLevel::Info, format!("Start to Parse: {}", item_name));
        if item_name.contains(".ogg") || item_name.contains(".caf") {
            let final_path = PathBuf::from(output_path)
                .join(get_folder_name(cc))
                .join("server")
                .join("Audio")
                .join(item_name.clone());

            create_dir(final_path.parent().unwrap().to_str().unwrap())?;

            let fp_str = final_path.to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid file path")
            })?;

            create_file(&read_file_from_zip(&mut zip, &item_name)?, fp_str)?;
        } else {
            let item_list_data = read_file_from_zip(&mut zip, &format!("{}.list", item_name))?;
            let item_pack_data = read_file_from_zip(&mut zip, &format!("{}.pack", item_name))?;

            let result =
                aes_decrypt::decrypt_ecb(false, &item_list_data.as_slice()).map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, format!("Decrypt error: {}", e))
                })?;

            let list_str = String::from_utf8(result).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("UTF-8 decode error: {}", e),
                )
            })?;

            for (i, line) in list_str.lines().enumerate().skip(1) {
                match Item::from_line(line) {
                    Ok(item) => {
                        let content = &item_pack_data[item.start..item.start + item.arrange];

                        let final_path = PathBuf::from(output_path)
                            .join(get_folder_name(cc))
                            .join("server")
                            .join(item_name.clone())
                            .join(&item.name);

                        item.write_file(&item_name, content, final_path)?;
                    }
                    Err(e) => {
                        log(
                            LogLevel::Warning,
                            format!("Invalid line format at line {}: {}", i + 1, e),
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
