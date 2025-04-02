use std::{ffi::OsStr, fs::File, io::Read};

use zip::ZipArchive;

use super::apk_parser::{self};
use crate::functions::file_selector::{self, file_dialog};
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::valid_apk::valid_pack::{valid_apk, valid_xapk};

pub fn dump_apk(update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let (apk, output_path) = match update {
        Some(true) => {
            let apk = std::env::temp_dir().join("temp.xapk");
            let mut cwd = std::env::current_dir()?.to_str().unwrap().to_string();

            cwd.push_str("\\Data\\Local\\");

            (apk, cwd)
        }
        _ => {
            println!("請選擇安裝檔 (.apk/.xapk)");
            let file = file_selector::file_dialog(
                true,
                Some("BC Apk".to_string()),
                Some(vec!["apk", "xapk"]),
            );

            let apk = match file {
                Some(f) => f,
                None => {
                    log(LogLevel::Error, "No file selected.".to_string());
                    return Ok(());
                }
            };

            log(
                LogLevel::Info,
                format!("Selected file: {}", apk.to_string_lossy()),
            );

            println!("請選擇輸出目錄");
            let output_path = file_dialog(false, None, None)
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "No folder selected")
                })?
                .to_str()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path")
                })?
                .to_string();

            log(
                LogLevel::Info,
                format!("Selected output folder: {}", output_path),
            );

            (apk, output_path)
        }
    };

    log(LogLevel::Info, "Start to get event data".to_string());

    match apk.extension().and_then(OsStr::to_str) {
        Some("apk") => {
            valid_apk()?;
        }
        Some("xapk") => {
            if let Ok(Some(package)) = valid_xapk(&apk) {
                let cc = match package.as_str() {
                    "jp.co.ponos.battlecats" => "jp",
                    _ => &package[package.len().saturating_sub(2)..],
                };

                let file = File::open(&apk)?;
                let mut zip = ZipArchive::new(file)?;

                let mut install_pack = zip.by_name("InstallPack.apk")?;
                let mut install_pack_data = Vec::new();
                install_pack.read_to_end(&mut install_pack_data)?;

                let temp_path = std::env::temp_dir().join("InstallPack.apk");
                std::fs::write(&temp_path, install_pack_data)?;

                apk_parser::parse_apk(cc, &output_path)?;

                std::fs::remove_file(temp_path)?;
            } else {
                return Err("Not a valid xapk".into());
            }
        }
        _ => {
            log(LogLevel::Error, "Unsupported file type.".to_string());
        }
    }

    Ok(())
}
