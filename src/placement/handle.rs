use crate::functions::file_selector::file_dialog;
use crate::functions::writer::create_file;
use crate::placement::requests;

pub async fn get_announcement() -> Result<(), std::io::Error> {
    let output_path = file_dialog(false, None, None).unwrap()
        .to_str()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid output path"))?
        .to_string();

    for cc in ["jp", "tw", "en", "kr"] {
        let result = requests::get_placement(cc)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let json = serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&result)?
        )?;

        let path = format!("{}/{}_placement.json", output_path, cc);

        create_file(json.as_bytes(), &path)?;
    }

    Ok(())
}
