use crate::event::get_token::EventData;
use crate::functions::file_selector::file_dialog;
use crate::functions::logger::logger::{log, LogLevel};

pub async fn get_data() -> Result<(), std::io::Error> {
    println!("請選擇輸出資料夾");
    let output_path = file_dialog(false, None, None)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No folder selected"))?
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path"))?
        .to_owned();
    log(
        LogLevel::Info,
        format!("Selected output folder: {}", output_path),
    );

    for cc in ["jp", "tw", "en", "kr"] {
        let mut event = EventData {
            account_code: None,
            password: None,
            password_refresh_token: None,
            jwt_token: None,
        };

        for file in ["sale", "gatya", "item"] {
            let _ = event.to_file(output_path.clone(), cc, file).await?;
        }
    }

    Ok(())
}
