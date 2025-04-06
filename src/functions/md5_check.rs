use md5;
use std::fs::File;
use std::io::{self, Read};

pub fn get_hash(data: impl AsRef<[u8]>) -> Result<String, io::Error> {
    let bytes = match data.as_ref() {
        path if std::path::Path::new(std::str::from_utf8(path).unwrap_or("")).exists() => {
            let mut file = File::open(std::str::from_utf8(path).unwrap())?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            buffer
        }
        _ => data.as_ref().to_vec(),
    };

    Ok(format!("{:x}", md5::compute(bytes)))
}
