use std::{ffi::OsStr, fs::File, io::Read};
use zip::ZipArchive;

use super::valid_pack::valid_pack::{valid_apk, valid_xapk};
use crate::{
    functions::{
        file_selector::{self, file_dialog},
        logger::logger::{log, LogLevel},
    },
    local::apk_parser,
};

pub fn dump_apk() {
    println!("請選擇安裝檔 (.apk/.xapk)");
    let file = file_selector::file_dialog(
        true,
        Some("BC Apk".to_string()),
        Some(["apk", "xapk"].to_vec()),
    );
    log(
        LogLevel::Info,
        format!("Selected file: {}", file.clone().unwrap().to_string_lossy()),
    );

    let output_path = file_dialog(false, None, None)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No folder selected"))
        .unwrap()
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path"))
        .unwrap()
        .to_owned();
    log(
        LogLevel::Info,
        format!("Selected output folder: {}", output_path),
    );

    if let Some(apk) = file {
        match apk.extension().and_then(OsStr::to_str) {
            Some("apk") => {
                valid_apk();
            }
            Some("xapk") => {
                if let Ok(Some(package)) = valid_xapk(&apk) {
                    let cc = if package == "jp.co.ponos.battlecats" {
                        "jp"
                    } else {
                        &package[package.len() - 2..]
                    };
                    log(LogLevel::Info, format!("Package Name: {}", package));

                    let mut zip = ZipArchive::new(File::open(apk).unwrap()).unwrap();

                    let mut install_pack = zip.by_name("InstallPack.apk").unwrap();

                    let mut install_pack_data = Vec::new();
                    install_pack.read_to_end(&mut install_pack_data).unwrap();

                    std::fs::write("InstallPack.apk", install_pack_data).unwrap();

                    let _ = apk_parser::parse_apk(cc, output_path.as_str());

                    std::fs::remove_file("InstallPack.apk").unwrap();
                } else {
                    panic!("Not a valid xapk");
                }
            }
            _ => {}
        }
    }
}
