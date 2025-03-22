pub mod valid_pack {
    use std::{fs::File, io::Read, path::PathBuf};

    use zip::ZipArchive;

    pub fn valid_apk() {
        todo!("Not implemented yet");
    }

    pub fn valid_xapk(path: &PathBuf) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut zip = ZipArchive::new(file)?;

        let mut manifest = zip.by_name("manifest.json")?;
        let mut manifest_str = String::new();
        manifest.read_to_string(&mut manifest_str)?;

        let manifest: serde_json::Value = serde_json::from_str(&manifest_str)?;

        if manifest["package_name"]
            .to_string()
            .contains("jp.co.ponos.battlecats")
        {
            Ok(Some(manifest["package_name"].to_string().replace("\"", "")))
        } else {
            Ok(None)
        }
    }
}
