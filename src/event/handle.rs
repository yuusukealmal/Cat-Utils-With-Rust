use std::path::PathBuf;

use crate::config::structs::Event;
use crate::functions::file_selector::file_dialog;
use crate::functions::git::{commit_or_push, Method};
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::utils::get_folder_name;

pub async fn get_event_data(update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
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

    log(LogLevel::Info, "Start to get event data".to_string());

    let mut event = Event::new();

    event.generate_account().await?;
    event.generate_jwtoken().await?;

    for cc in ["jp", "tw", "en", "kr"] {
        let path = PathBuf::from(&output_path)
            .join(get_folder_name(cc))
            .join("event");

        event.output_path = Some(path);

        for file in ["sale", "gatya", "item"] {
            event.to_file(cc, file).await?;
        }
        if update.unwrap_or(false) {
            commit_or_push(
                Method::COMMIT,
                Some(&format!(
                    "Update Certain Game {} Event Data",
                    cc.to_uppercase()
                )),
            )?;
        }
    }

    Ok(())
}
