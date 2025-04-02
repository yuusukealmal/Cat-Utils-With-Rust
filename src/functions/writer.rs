use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, Result};

pub fn create_dir(path: &str) -> Result<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn create_file(data: &[u8], filename: &str) -> Result<()> {
    if let Some(parent) = Path::new(filename).parent() {
        create_dir(parent.to_str().unwrap())?;
    }

    let mut file = File::create(filename)?;
    file.write_all(data)?;

    Ok(())
}
