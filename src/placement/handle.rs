use std::path::PathBuf;

use super::requests;
use crate::functions::file_selector::file_dialog;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::create_file;

pub async fn get_announcement(update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = match update {
        Some(true) => {
            let cwd = std::env::current_dir()?;
            let mut output_path = cwd.to_str().unwrap().to_string();
            output_path.push_str("\\Data\\Placement");

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

    for cc in ["jp", "tw", "en", "kr"] {
        let result = requests::get_placement(cc)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let json =
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&result)?)?;

        let path = PathBuf::from(&output_path).join(format!("{cc}_placement.json"));
        create_file(json.as_bytes(), &path.to_string_lossy())?;
    }

    Ok(())
}
