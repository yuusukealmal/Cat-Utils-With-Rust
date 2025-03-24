use std::{fs::File, io::Read};

use zip::ZipArchive;

use crate::functions::logger::logger::{log, LogLevel};
use crate::server::{get_version, zip_download};

pub async fn parse_server(
    cc: &str,
    output_path: &str,
    zip: &mut ZipArchive<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    if cc == "en" {
        log(
            LogLevel::Error,
            "Not implemented Yet, Cause I'm lazy".to_string(),
        );
    } else {
        let architectures = vec![
            "x86",
            "x86_64",
            "arm64_v8a",
            "armeabi_v7a",
            "armeabi",
            "mips",
            "mips64",
        ];

        let zip_file_names: Vec<String> = zip.file_names().map(|name| name.to_string()).collect();

        'outer: for arch in architectures {
            for file_name in &zip_file_names {
                if file_name.contains(arch) {
                    let mut lib = zip.by_name(file_name)?;
                    let mut lib_data = Vec::new();
                    lib.read_to_end(&mut lib_data)?;

                    log(LogLevel::Info, format!("Get Architecture: {}", arch));
                    let temp_path = std::env::temp_dir().join("lib.so");
                    std::fs::write(&temp_path, lib_data)?;

                    break 'outer;
                }
            }
        }

        let versions = get_version::get_version(cc)?;

        log(
            LogLevel::Info,
            format!("Get Versions Length: {}", versions.len()),
        );

        for (index, version) in versions.iter().enumerate() {
            zip_download::download_zip(cc, index, version).await?;
            break;
        }
    }

    Ok(())
}
