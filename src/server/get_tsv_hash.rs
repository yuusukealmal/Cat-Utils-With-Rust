use std::io::{Cursor, Read};

use zip::ZipArchive;

use crate::config::structs::ServerAPK;

impl ServerAPK {
    pub fn get_tsv_hash(&mut self) -> Result<(Vec<String>, Vec<i32>), Box<dyn std::error::Error>> {
        let mut hashmap = Vec::new();
        let mut region_cnt = vec![0; 6];

        let mut apk_data = Vec::new();
        let mut apk_file = self.zip.by_name("InstallPack.apk")?;
        apk_file.read_to_end(&mut apk_data)?;

        let reader = Cursor::new(apk_data);
        let mut apk_zip = ZipArchive::new(reader)?;

        let mut tsv_files: Vec<String> = apk_zip
            .file_names()
            .filter(|name| name.starts_with("assets/download") && name.ends_with(".tsv"))
            .map(String::from)
            .collect();

        if &self.cc == "en" {
            let region_order = vec!["", "fr", "it", "de", "es", "th"];
            tsv_files.sort_by_key(|name| {
                let region = name
                    .split('_')
                    .nth(0)
                    .unwrap()
                    .replace("assets/download", "");
                let index = region_order
                    .iter()
                    .position(|&r| r == region)
                    .unwrap_or(region_order.len());
                let num: i32 = name
                    .split('_')
                    .last()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                (index, num)
            });

            for name in &tsv_files {
                let region = name
                    .split('_')
                    .nth(0)
                    .unwrap()
                    .replace("assets/download", "");
                let index = region_order
                    .iter()
                    .position(|&r| r == region)
                    .unwrap_or(region_order.len());
                region_cnt[index] += 1;
            }
        } else {
            tsv_files.sort_by_key(|name| {
                name.split('_')
                    .last()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            });
        }

        for name in tsv_files {
            let mut file_data = String::new();
            apk_zip.by_name(&name)?.read_to_string(&mut file_data)?;

            if let Some(first_line) = file_data.lines().next() {
                let columns: Vec<&str> = first_line.split('\t').collect();
                if columns.len() > 2 {
                    hashmap.push(columns[2].to_string());
                }
            }
        }

        Ok((hashmap, region_cnt))
    }
}
