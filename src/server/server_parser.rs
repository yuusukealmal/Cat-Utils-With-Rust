use std::fs;
use std::io::Read;
use std::path::Path;

use serde_json::{Map, Value};

use crate::config::structs::ServerAPK;
use crate::functions::json_prettier::indent_json;

impl ServerAPK {
    pub async fn parse_server(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
        let zip_file_names: Vec<String> =
            self.zip.file_names().map(|name| name.to_string()).collect();

        if let Some(arch) = architectures.iter().find(|&&arch| {
            zip_file_names
                .iter()
                .any(|file_name| file_name.contains(arch))
        }) {
            for file_name in &zip_file_names {
                if file_name.contains(*arch) {
                    let mut lib = self.zip.by_name(file_name)?;
                    let mut lib_data = Vec::new();
                    lib.read_to_end(&mut lib_data)?;

                    std::fs::write(&temp_path, lib_data)?;
                    break;
                }
            }
        }

        let versions = self.get_version()?;
        let tsvs = self.get_tsv_hash()?;

        if let Some(true) = self.update {
            let data: Map<String, Value> = serde_json::from_str(&fs::read_to_string("data.json")?)?;
            let server_versions = data[&self.cc.to_uppercase()].as_object().unwrap()["server"]
                .as_object()
                .unwrap();
            let mut data_mut = data.clone();

            for (index, version) in versions.iter().enumerate() {
                let current_version = server_versions
                    .get(&format!("assets{}", index))
                    .unwrap_or(&serde_json::Value::Null);

                if current_version != &tsvs.0[index] {
                    self.download_zip(index, version, &tsvs).await?;
                    self.parse_zip()?;
                    data_mut[&self.cc.to_uppercase()]["server"][&format!("assets{}", index)] =
                        serde_json::Value::String(tsvs.0[index].clone());
                }

                fs::write("data.json", indent_json(&data_mut)?)?;
            }
        } else {
            for (index, version) in versions.iter().enumerate() {
                self.download_zip(index, version, &tsvs).await?;
                self.parse_zip()?;
            }
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
}
