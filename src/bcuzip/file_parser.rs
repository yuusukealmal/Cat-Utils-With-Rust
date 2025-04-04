pub mod length_count {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    use std::result::Result;

    use md5;

    use crate::config::structs::{BCUFiles, BCUZIP};
    use crate::functions::logger::logger::{log, LogLevel};
    use crate::functions::writer::writer::create_dir;

    fn cnt_length(fp: &str) -> Result<BCUZIP, std::io::Error> {
        let mut file = File::open(fp)?;
        let mut buffer = vec![0; 0x24];

        file.read_exact(&mut buffer)?;

        let length = u32::from_le_bytes(buffer[0x20..0x24].try_into().unwrap());
        let pad = 16 * (length / 16 + 1);

        let datalength = 0x24 + pad as u64;
        let file_len = file.metadata()?.len();
        let mut data_buffer = vec![0; (file_len - datalength) as usize];

        file.seek(SeekFrom::Start(datalength))?;
        file.read_exact(&mut data_buffer)?;

        let key_buffer = <[u8; 16]>::try_from(&buffer[0x10..0x20]).unwrap();

        let hash = md5::compute(b"battlecatsultimate");
        let iv = <[u8; 16]>::try_from(&hash[..16]).unwrap();

        Ok(BCUZIP {
            title: String::new(),
            length,
            pad,
            data: data_buffer,
            key: key_buffer,
            iv,
        })
    }

    pub fn parse_file(fp: &str, dest: &str) -> Result<(), std::io::Error> {
        let mut zip = cnt_length(fp)?;
        let mut file = File::open(fp)?;

        let info_str = BCUZIP::write_info(&zip, &mut file, dest)?;
        let info: serde_json::Value = serde_json::from_str(&info_str).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "JSON parse error")
        })?;

        zip.title = info["desc"]["id"]
            .as_str()
            .or_else(|| info["desc"]["names"]["dat"][0]["val"].as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        log(LogLevel::Info, format!("Title: {}", zip.title));

        create_dir(&format!("{}/{}", dest, zip.title))?;

        for item in info["files"].as_array().unwrap_or(&vec![]).iter() {
            if let Some(obj) = item.as_object() {
                let f = BCUFiles {
                    offset: obj.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                    path: obj
                        .get("path")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    size: obj.get("size").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
                };

                BCUZIP::write_file(&zip, &f, dest)?;
            }
        }
        println!();

        Ok(())
    }
}
