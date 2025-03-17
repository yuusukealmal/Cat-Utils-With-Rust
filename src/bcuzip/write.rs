pub mod base_functions {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Result;

    pub fn create_dir(path: &str) -> Result<()> {
        std::fs::create_dir_all(path).unwrap();
        Ok(())
    }

    pub fn create_file(data: &[u8], filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(data)?;

        Ok(())
    }
}

pub mod write_functions {
    use std::fs::File;
    use std::io::Result;
    use std::io::{self, Read, Seek, SeekFrom};
    use std::path::{Path, PathBuf};

    use serde_json;
    
    use crate::bcuzip::aes_decrypt::aes;
    use crate::bcuzip::file_parser::{Files, BCUZIP};

    use super::base_functions::{create_dir, create_file};

    pub fn write_info(zip: &BCUZIP, file: &mut File, dest: &str) -> Result<String> {
        let mut info_buffer = vec![0; zip.pad as usize];
        file.seek(SeekFrom::Start(0x24))?;
        file.read(&mut info_buffer)?;

        let info = aes::aes_pack(&zip, zip.length, &info_buffer)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

        let info = String::from_utf8(info).unwrap();

        let mut infofp = dest.to_string();
        infofp.push_str("/info.json");

        let json_value: serde_json::Value = serde_json::from_str(&info)?;
        let json_str = serde_json::to_string_pretty(&json_value)?;

        let json_bytes = json_str.as_bytes();

        create_file(json_bytes, &infofp)?;

        Ok(info)
    }

    pub fn write_file(zip: &BCUZIP, file: &Files, dest: &str) -> Result<()> {
        let file_name = file.path.split("/").last().unwrap();

        let base_path = Path::new(dest);
        let title = Path::new(zip.title.as_str());
        let relative_path = PathBuf::from(file.path.as_str());

        let fp = base_path.join(title).join(relative_path);

        create_dir(fp.parent().unwrap().to_str().unwrap())?;

        let pad = file.size + (16 - file.size % 16);
        let slice_data = &zip.data[file.offset as usize..(file.offset + pad) as usize];
        let data = aes::aes_pack(&zip, file.size, slice_data).unwrap();

        let output_path = fp.to_str().unwrap();

        if file_name != "pack.json" {
            create_file(&data, output_path)?;
        } else {
            let json_value: serde_json::Value = serde_json::from_str(String::from_utf8(data).unwrap().as_str()).unwrap();
            let json_str = serde_json::to_string_pretty(&json_value)?;
            let json_bytes = json_str.as_bytes();
    
            create_file(json_bytes, output_path)?;
        }

        Ok(())
    }
}
// self.data[offset:offset + mod]
