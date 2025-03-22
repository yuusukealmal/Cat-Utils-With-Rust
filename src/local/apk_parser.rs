use std::{env, fs::File};

use zip::ZipArchive;

#[derive(Debug)]
pub struct APK {
    pub cc: String,
    pub list_key: String,
    pub pack_key: String,
    pub pack_iv: String,
    pub items: Vec<String>,
}
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub start: usize,
    pub arrange: usize,
}

impl APK {
    fn read_items(&mut self) -> Result<(), std::io::Error> {
        let file = File::open("InstallPack.apk")?;
        let zip = ZipArchive::new(file)?;

        zip.file_names().for_each(|name| {
            if name.ends_with(".list") {
                self.items.push(name.to_string().replace(".list", ""));
            }
        });

        Ok(())
    }
}

pub fn parse_apk(cc: &str, output_path: &str) {
    let mut apk = APK {
        cc: cc.to_string(),
        list_key: env::var("LIST").unwrap(),
        pack_key: env::var(format!("{}_PACK", cc)).unwrap(),
        pack_iv: env::var(format!("{}_IV", cc)).unwrap(),
        items: Vec::new(),
    };

    let _ = apk.read_items();

    for item in apk.items.clone() {
        let _ = apk.parse_item(output_path, &item);
    }
}
