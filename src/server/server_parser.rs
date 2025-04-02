use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use serde_json::{Map, Value};
use zip::ZipArchive;

use super::{get_tsv_hash, get_version, parse_zip, zip_download};
use crate::functions::json_prettier::indent_json;
use crate::functions::logger::logger::{log, LogLevel};

pub async fn parse_server(
    update: Option<bool>,
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

    let data: Map<String, Value> = serde_json::from_str(&fs::read_to_string("data.json")?)?;
    let server_versions = data[&cc.to_uppercase()].as_object().unwrap()["server"]
        .as_object()
        .unwrap();

    let tsvs = get_tsv_hash::get_tsv_hash(cc)?;
    let mut data_mut = data.clone();

    for (index, version) in versions.iter().enumerate() {
        if server_versions[&format!("assets{}", index)] != tsvs[index] {
            zip_download::download_zip(cc, index, version).await?;
            parse_zip::parse_zip(cc, output_path)?;
            data_mut[&cc.to_uppercase()]["server"][&format!("assets{}", index)] =
                serde_json::Value::String(tsvs[index].clone());
        }
    }

    if let Some(true) = update {
        fs::write("data.json", indent_json(&data_mut)?)?;
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
