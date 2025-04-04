use crate::config::routes::EVENT_FILE;
use crate::config::structs::Event;
use crate::functions::writer::writer::create_file;

impl Event {
    pub async fn to_file(&self, cc: &str, file: &str) -> Result<(), std::io::Error> {
        let cc_suffix = if cc == "jp" { "" } else { cc };

        let url = EVENT_FILE(cc_suffix, file, self.jwt_token.as_deref().unwrap());

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
