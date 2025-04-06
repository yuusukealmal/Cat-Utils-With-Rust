use std::fs::File;

use zip::ZipArchive;

use crate::config::structs::APK;

impl APK {
    fn read_items(&self) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(std::env::temp_dir().join("InstallPack.apk"))?;
        let zip = ZipArchive::new(file)?;

        let items = zip
            .file_names()
            .filter(|name| name.ends_with(".list"))
            .map(|name| name.replace(".list", ""))
            .collect();

        Ok(items)
    }

    pub fn parse_apk(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut items = self.read_items()?;
        items.retain(|s| !s.contains("HtmlLocal"));
        for item in &items {
            self.parse_item(item)?;
        }

        Ok(())
    }
}
