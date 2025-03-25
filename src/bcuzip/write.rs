use std::fs::File;
use std::io::Result;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;

use serde_json;

use super::file_parser::Files;
use crate::functions::writer::{create_dir, create_file};

pub struct BCUZIP {
    pub title: String,
    pub length: u32,
    pub pad: u32,
    pub data: Vec<u8>,
    pub key: [u8; 16],
    pub iv: [u8; 16],
}

impl BCUZIP {
    pub fn write_info(&self, file: &mut File, dest: &str) -> Result<String> {
        let mut info_buffer = vec![0; self.pad as usize];
        file.seek(SeekFrom::Start(0x24))?;
        file.read_exact(&mut info_buffer)?;

        let info = self.aes_pack(self.length as usize, &info_buffer);

        let info = String::from_utf8(info.unwrap())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let infofp = format!("{}/info.json", dest);

        create_file(
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&info)?)?
                .as_bytes(),
            &infofp,
        )?;

        Ok(info)
    }

    pub fn write_file(&self, file: &Files, dest: &str) -> Result<()> {
        let file_name = file.path.split("/").last().unwrap();

        let fp = PathBuf::from(dest).join(&self.title).join(&file.path);

        create_dir(fp.parent().unwrap().to_str().unwrap())?;

        let pad = file.size + (16 - file.size % 16);
        let slice_data = &self.data[file.offset as usize..(file.offset + pad) as usize];
        let data = self.aes_pack(file.size as usize, slice_data).unwrap();

        let output_path = fp.to_str().unwrap();

        if file_name == "pack.json" {
            let json_str =
                serde_json::to_string_pretty(&serde_json::from_slice::<serde_json::Value>(&data)?)?;
            create_file(json_str.as_bytes(), output_path)?;
        } else {
            create_file(&data, output_path)?;
        }

        Ok(())
    }
}
