use super::get_token::EventData;
use crate::functions::file_selector::file_dialog;
use crate::functions::git::{commit_or_push, Method};
use crate::functions::logger::logger::{log, LogLevel};

pub async fn get_event_data(update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = match update {
        Some(true) => {
            let cwd = std::env::current_dir()?;
            let mut output_path = cwd.to_str().unwrap().to_string();
            output_path.push_str("\\Data\\Event");

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

    let mut event = EventData {
        account_code: None,
        password: None,
        password_refresh_token: None,
        jwt_token: None,
    };

    event.generate_account().await.ok();
    event.generate_jwtoken().await.ok();

    for cc in ["jp", "tw", "en", "kr"] {
        for file in ["sale", "gatya", "item"] {
            event.to_file(output_path.clone(), cc, file).await?;
        }
        if update.unwrap_or(false) {
            commit_or_push(Method::COMMIT, Some(&format!("Update Certain Game {} Event Data", cc.to_uppercase())))?;
        }
    }

    Ok(())
}
