pub mod valid_pack {
    use std::{fs::File, io::Read, path::PathBuf};
    use zip::ZipArchive;

    use crate::functions::logger::logger::{log, LogLevel};

    pub fn valid_apk() -> Result<Option<String>, Box<dyn std::error::Error>> {
        log(LogLevel::Warning, "Not implemented Yet".to_string());
        Ok(None)
    }

    pub fn valid_xapk(path: &PathBuf) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut zip = ZipArchive::new(file)?;

        let mut manifest = zip
            .by_name("manifest.json")
            .map_err(|_| "Missing manifest.json")?;

        let mut manifest_str = String::new();
        manifest.read_to_string(&mut manifest_str)?;

        let manifest: serde_json::Value = serde_json::from_str(&manifest_str)?;

        if let Some(package_name) = manifest.get("package_name").and_then(|v| v.as_str()) {
            if package_name.contains("jp.co.ponos.battlecats") {
                return Ok(Some(package_name.to_string()));
            }
        }
        Ok(None)
    }
}
