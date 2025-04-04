use std::path::PathBuf;

use serde_json::{Map, Value};

use super::requests;
use crate::functions::file_selector::file_dialog;
use crate::functions::git::{commit_or_push, Method};
use crate::functions::json_prettier::indent_json;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::create_file;

pub async fn get_announcement(update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = match update {
        Some(true) => {
            let cwd = std::env::current_dir()?;
            let mut output_path = cwd.to_str().unwrap().to_string();
            output_path.push_str("\\Data");

            output_path
        }
        _ => {
            println!("請選擇輸出資料夾");
            let output_path = file_dialog(false, None, None)
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "No folder selected")
                })?
                .to_str()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path")
                })?
                .to_owned();
            log(
                LogLevel::Info,
                format!("Selected output folder: {}", output_path),
            );

            output_path
        }
    };

    log(LogLevel::Info, "Start to get announcement data".to_string());

    for cc in ["jp", "tw", "en", "kr"] {
        let cc_suffix = match cc {
            "jp" => "",
            _ => &format!("/{}", cc),
        };

        let folder_name = match cc {
            "jp" => "にゃんこ大戦争",
            "tw" => "貓咪大戰爭",
            "en" => "The Battle Cats",
            "kr" => "냥코대전쟁",
            _ => "Unknown",
        };

        let result = requests::get_placement(cc)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let json: Map<String, Value> = serde_json::from_str(&result)?;

        let path = PathBuf::from(&output_path)
            .join(&folder_name)
            .join("placement")
            .join("placement.json");

        create_file(indent_json(&json)?.as_bytes(), path.to_str().unwrap())?;

        let json: serde_json::Value = serde_json::from_str(&result)?;

        for notice in json["notice"]["data"].as_array().unwrap() {
            let uuid = notice["id"].as_str().unwrap();

            let url = String::from(format!(
                "https://ponosgames.com/information/appli/battlecats/placement{}/notice_{}.png",
                cc_suffix, uuid
            ));

            let data = reqwest::get(&url).await.unwrap().bytes().await.unwrap();

            let path = PathBuf::from(&output_path)
                .join(&folder_name)
                .join("placement")
                .join("picture")
                .join(format!("{}.png", uuid));

            create_file(&data, &path.to_string_lossy())?;
        }

        if update.unwrap_or(false) {
            commit_or_push(
                Method::COMMIT,
                Some(&format!(
                    "Update Certain Game  {} Announcement",
                    cc.to_uppercase()
                )),
            )?;
        }
    }

    Ok(())
}
