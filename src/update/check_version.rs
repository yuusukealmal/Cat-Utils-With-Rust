use std::fs;

use regex::Regex;
use reqwest;
use select::{document::Document, predicate::Attr};
use ua_generator::ua::spoof_ua;

use crate::functions::utils::parse_version_int;

async fn requests_version(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ua = spoof_ua();

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header("User-Agent", ua)
        .send()
        .await?
        .text()
        .await?;

    let document = Document::from(body.as_str());

    let content = document
    .find(Attr("name", "description"))
    .next()
    .and_then(|meta| meta.attr("content"))
    .ok_or("No meta description found")?;

    let re = Regex::new(r"\b\d+\.\d+\.\d+\b")?;
    let version = re.find(content).ok_or("Version not found")?.as_str();

    Ok(version.to_string())
}

pub async fn check_version() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut result = vec![];
    let data_json: serde_json::Value = serde_json::from_str(&fs::read_to_string("data.json")?)?;

    for cc in ["JP", "TW", "KR"] {
        //pass "EN"
        let current_version = data_json[cc]["version"].as_u64().unwrap();
        let latest_version = requests_version(
            data_json[cc.to_uppercase()].as_object().unwrap()["version_url"]
                .as_str()
                .unwrap(),
        )
        .await?;

        if parse_version_int(&latest_version)? as u64 > current_version {
            result.push((cc.to_string(), latest_version));
        }
    }

    Ok(result)
}
