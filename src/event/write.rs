use std::path::PathBuf;

use super::get_token::EventData;
use crate::functions::utils::get_folder_name;
use crate::functions::writer::create_file;

impl EventData {
    pub async fn to_file(
        &mut self,
        output_path: String,
        cc: &str,
        file: &str,
    ) -> Result<(), std::io::Error> {
        let cc_suffix = if cc == "jp" { "" } else { cc };

        let url = format!(
            "https://nyanko-events.ponosgames.com/battlecats{cc_suffix}_production/{file}.tsv?jwt={}",
            self.jwt_token.as_deref().unwrap_or("")
        );

        let data = reqwest::get(&url).await.unwrap().text().await.unwrap();

        let path = PathBuf::from(&output_path)
            .join(get_folder_name(cc))
            .join("event")
            .join(&format!("{}.tsv", file));

        create_file(data.as_bytes(), &path.to_string_lossy())?;

        Ok(())
    }
}
