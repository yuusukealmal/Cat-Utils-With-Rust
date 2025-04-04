use crate::config::structs::Event;
use crate::functions::writer::writer::create_file;

impl Event {
    pub async fn to_file(&mut self, cc: &str, file: &str) -> Result<(), std::io::Error> {
        let cc_suffix = if cc == "jp" { "" } else { cc };

        let url = format!(
            "https://nyanko-events.ponosgames.com/battlecats{cc_suffix}_production/{file}.tsv?jwt={}",
            self.jwt_token.as_deref().unwrap_or("")
        );

        let data = reqwest::get(&url).await.unwrap().text().await.unwrap();

        let path = self
            .output_path
            .as_ref()
            .unwrap()
            .join(&format!("{}.tsv", file));

        create_file(data.as_bytes(), &path.to_string_lossy())?;

        Ok(())
    }
}
