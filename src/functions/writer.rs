use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn create_dir(path: &str) -> Result<()> {
    std::fs::create_dir_all(path)?;

    Ok(())
}

pub fn create_file(data: &[u8], filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data)?;

    Ok(())
}
