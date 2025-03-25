use std::path::PathBuf;

use super::requests;
use crate::functions::file_selector::file_dialog;
use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::writer::create_file;

pub async fn get_announcement() -> Result<(), std::io::Error> {
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
        let result = requests::get_placement(cc)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let json =
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&result)?)?;

        let path = PathBuf::from(&output_path).join(format!("{cc}_placement.json"));
        create_file(json.as_bytes(), &path.to_string_lossy())?;
        log(
            LogLevel::Info,
            format!("Success Write Placement to {}", path.to_string_lossy()),
        );
    }

    Ok(())
}
