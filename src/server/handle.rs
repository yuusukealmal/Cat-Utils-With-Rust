use std::{ffi::OsStr, fs::File};
use zip::ZipArchive;

use crate::functions::valid_apk::valid_pack::{valid_apk, valid_xapk};
use crate::functions::{
    file_selector::{self, file_dialog},
    logger::logger::{log, LogLevel},
};
use crate::server::server_parser;

pub async fn get_server_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("請選擇安裝檔 (.apk/.xapk)");
    let file =
        file_selector::file_dialog(true, Some("BC Apk".to_string()), Some(vec!["apk", "xapk"]));

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

    let output_path = file_dialog(false, None, None)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No folder selected"))?
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path"))?
        .to_string();

    log(
        LogLevel::Info,
        format!("Selected output folder: {}", output_path),
    );

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

                log(LogLevel::Info, format!("Package Name: {}", package));

                let file = File::open(&apk)?;
                let mut zip = ZipArchive::new(file)?;

                server_parser::parse_server(cc, &output_path, &mut zip).await?;
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
