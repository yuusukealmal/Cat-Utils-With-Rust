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
}

pub fn parse_apk(cc: String, output_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(std::env::temp_dir().join("InstallPack.apk"))?;
    let zip = ZipArchive::new(file)?;

    let mut apk = APK {
        cc: cc.to_string(),
        output_path: output_path.to_string(),
        zip,
    };

    let items = apk.read_items()?;
    for item in &items {
        apk.parse_item(item)?;
    }

    Ok(())
}
