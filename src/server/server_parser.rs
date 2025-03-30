use std::{fs::File, io::Read, path::Path};

use zip::ZipArchive;

use super::parse_zip;
use super::{get_version, zip_download};
use crate::functions::logger::logger::{log, LogLevel};

pub async fn parse_server(
    cc: &str,
    output_path: &str,
    zip: &mut ZipArchive<File>,
) -> Result<(), Box<dyn std::error::Error>> {
    if cc == "en" {
        log(
            LogLevel::Error,
            "Not implemented yet, because I'm lazy".to_string(),
        );
        return Ok(());
    }

    let architectures = vec![
        "x86",
        "x86_64",
        "arm64_v8a",
        "armeabi_v7a",
        "armeabi",
        "mips",
        "mips64",
    ];

    let temp_path = std::env::temp_dir().join("lib.so");
    let zip_file_names: Vec<String> = zip.file_names().map(|name| name.to_string()).collect();

    if let Some(arch) = architectures.iter().find(|&&arch| {
        zip_file_names
            .iter()
            .any(|file_name| file_name.contains(arch))
    }) {
        for file_name in &zip_file_names {
            if file_name.contains(*arch) {
                let mut lib = zip.by_name(file_name)?;
                let mut lib_data = Vec::new();
                lib.read_to_end(&mut lib_data)?;

                std::fs::write(&temp_path, lib_data)?;
                break;
            }
        }
    }

    let versions = get_version::get_version(cc)?;

    for (index, version) in versions.iter().enumerate() {
        zip_download::download_zip(cc, index, version).await?;
        log(LogLevel::Info, format!("Start to Parse {}", version));
        parse_zip::parse_zip(cc, output_path)?;
    }

    if Path::new(&temp_path).exists() {
        std::fs::remove_file(temp_path)?;
    }

    let temp_zip_path = std::env::temp_dir().join("temp.zip");
    if Path::new(&temp_zip_path).exists() {
        std::fs::remove_file(temp_zip_path)?;
    }

    Ok(())
}
