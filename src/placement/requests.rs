use crate::config::routes::{PLACEMENT_JSON, PLACEMENT_PNG};

pub async fn get_placement(cc: &str) -> reqwest::Result<String> {
    let cc_suffix = if cc == "jp" { "" } else { cc };
    let url = PLACEMENT_JSON(cc_suffix);

    reqwest::get(&url).await?.text().await
}

pub async fn get_pictures(cc_suffix: &str, uuid: &str) -> reqwest::Result<bytes::Bytes> {
    let url = PLACEMENT_PNG(cc_suffix, uuid);

    reqwest::get(&url).await.unwrap().bytes().await
}
