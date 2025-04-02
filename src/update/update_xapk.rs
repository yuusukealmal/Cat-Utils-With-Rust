use std::fs;

use serde_json::{Map, Value};

use super::check_version::check_version;
use super::download_apk::download_apk;
use crate::functions::json_prettier::indent_json;
use crate::functions::utils::parse_version_int;
use crate::local::handle::dump_apk;
use crate::server::handle::get_server_file;

pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    let to_update = check_version().await?;

    if to_update.is_empty() {
        return Ok(());
    }

    for cc in to_update {
        match download_apk(&cc.0).await {
            Ok(_) => {
                dump_apk(Some(true))?;
                get_server_file(Some(true)).await?;

                let mut data: Map<String, Value> =
                    serde_json::from_str(&fs::read_to_string("data.json")?)?;
                data[&cc.0]["version"] = Value::Number(parse_version_int(&cc.1)?.into());
                fs::write("data.json", indent_json(&data)?)?;
            }
            Err(e) => {
                println!("Error downloading XAPK for cc: {}: {}", cc.0, e);
            }
        }
    }

    fs::remove_file(std::env::temp_dir().join("temp.xapk")).unwrap();

    Ok(())
}
