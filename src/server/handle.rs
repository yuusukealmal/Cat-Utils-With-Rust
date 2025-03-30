use std::{ffi::OsStr, fs::File};

use zip::ZipArchive;

use super::server_parser;
use crate::functions::file_selector::{self, file_dialog};
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::valid_apk::valid_pack::{valid_apk, valid_xapk};

pub async fn get_server_file() -> Result<(), Box<dyn std::error::Error>> {
    println!("請選擇安裝檔 (.apk/.xapk)");
    let apk =
        file_selector::file_dialog(true, Some("BC Apk".to_string()), Some(vec!["apk", "xapk"]))
            .ok_or("未選擇檔案")?;

    log(
        LogLevel::Info,
        format!("Selected file: {}", apk.to_string_lossy()),
    );

    println!("請選擇輸出資料夾");
    let output_path = file_dialog(false, None, None)
        .ok_or("未選擇輸出資料夾")?
        .to_str()
        .ok_or("無效的輸出路徑")?
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
            let package = valid_xapk(&apk)?.ok_or("Invalid XAPK")?;

            let cc = match package.as_str() {
                "jp.co.ponos.battlecats" => "jp",
                _ => package
                    .get(package.len().saturating_sub(2)..)
                    .ok_or("Error Country Code")?,
            };

            let file = File::open(&apk)?;
            let mut zip = ZipArchive::new(file)?;

            server_parser::parse_server(cc, &output_path, &mut zip).await?;
        }
        _ => {
            log(LogLevel::Error, "Unsupported file type.".to_string());
        }
    }

    Ok(())
}
