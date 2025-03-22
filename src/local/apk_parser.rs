use std::fs::File;

use zip::ZipArchive;

pub struct APK {}

pub struct Item {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
}

impl APK {
    fn read_items(&self) -> Result<Vec<String>, std::io::Error> {
        let file = File::open("InstallPack.apk")?;
        let zip = ZipArchive::new(file)?;

        let items = zip
            .file_names()
            .filter(|name| name.ends_with(".list"))
            .map(|name| name.replace(".list", ""))
            .collect();

        Ok(items)
    }
}

pub fn parse_apk(cc: &str, output_path: &str) -> Result<(), std::io::Error> {
    let mut apk = APK {};
    let items = apk.read_items()?;
    for item in &items {
        let _ = apk.parse_item(cc, output_path, item);
    }

    Ok(())
}
