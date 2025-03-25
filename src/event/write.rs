use crate::event::get_token::EventData;
use crate::functions::logger::logger::{log, LogLevel};
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
        let file_path = format!("{}\\{}_{}.tsv", output_path, cc, file);
        create_file(data.as_bytes(), &file_path)?;
        log(
            LogLevel::Info,
            format!("Successfully wrote file: {}", file_path),
        );
        Ok(())
    }
}
