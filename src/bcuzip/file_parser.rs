#[derive(Debug)]
pub struct BCUZIP {
    pub title: String,
    pub length: u32,
    pub pad: u32,
    pub data: Vec<u8>,
    pub key: [u8; 16],
    pub iv: [u8; 16],
}

#[derive(Debug)]
pub struct Files {
    pub offset: u32,
    pub path: String,
    pub size: u32,
}

pub mod length_count {
    use md5;
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    use std::result::Result;

    use super::{Files, BCUZIP};
    use crate::bcuzip::write::base_functions::create_dir;
    use crate::bcuzip::write::write_functions::{write_file, write_info};

    fn cnt_length(fp: &str) -> Result<BCUZIP, std::io::Error> {
        let mut length_buffer: [u8; 4] = [0; 4];
        let mut file: File = File::open(fp)?;

        file.seek(SeekFrom::Start(0x20))?;
        file.read(&mut length_buffer)?;

        let length: u32 = u32::from_le_bytes(length_buffer);
        let pad: u32 = 16 * (length / 16 + 1);

        let datalength: u64 = 0x24 + pad as u64;
        let file_len: u64 = file.seek(SeekFrom::End(0))?;
        let mut data_buffer: Vec<u8> = vec![0; file_len as usize - datalength as usize];

        file.seek(SeekFrom::Start(datalength))?;
        file.read(&mut data_buffer)?;

        let mut key_buffer: [u8; 16] = [0; 16];
        file.seek(SeekFrom::Start(0x10))?;
        file.read(&mut key_buffer)?;

        let hash = md5::compute("battlecatsultimate".as_bytes());
        let mut iv: [u8; 16] = [0; 16];
        iv.copy_from_slice(&hash[..16]);

        let mut info_buffer = [];
        file.seek(SeekFrom::Start(0x20))?;
        file.read(&mut info_buffer)?;

        let zip = BCUZIP {
            title: String::from(""),
            length: length,
            pad: pad,
            data: data_buffer,
            key: key_buffer,
            iv: iv,
        };

        Ok(zip)
    }

    pub fn parse_file(fp: &str, dest: &str) -> Result<(), std::io::Error> {
        let mut zip = cnt_length(fp)?;
        let mut file = File::open(fp)?;

        let info = write_info(&zip, &mut file, dest)?;

        let info: serde_json::Value = serde_json::from_str(&info)?;

        if Some(info["desc"]["id"].as_str()) != None {
            zip.title = info["desc"]["id"].as_str().unwrap().to_string();
        } else {
            zip.title = info["desc"]["names"]["dat"][0]["val"]
                .as_str()
                .unwrap()
                .to_string();
        }

        create_dir(&format!("{}/{}", dest, zip.title))?;

        for i in info["files"].as_array().unwrap() {
            let obj = i.as_object().unwrap();
            let f = Files {
                offset: obj["offset"].as_u64().unwrap() as u32,
                path: obj["path"].as_str().unwrap().to_string(),
                size: obj["size"].as_u64().unwrap() as u32,
            };

            write_file(&zip, &f, dest)?;
        }

        Ok(())
    }
}
