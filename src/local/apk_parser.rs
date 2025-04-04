use std::{fs::File, path::PathBuf};

use zip::ZipArchive;

pub struct APK {
    pub cc: String,
    pub output_path: String,
    pub zip: ZipArchive<File>,
}

#[allow(dead_code)]
pub struct Item {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
    pub output_path: PathBuf,
}

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
