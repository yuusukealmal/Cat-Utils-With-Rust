use reqwest::header::HeaderMap;
use reqwest::Error as ReqwestError;
use serde_json::json;

use crate::functions::utils;

pub async fn get_save(
    account: &str,
    password: &str,
    version: u32,
    cc: &str,
) -> Result<Vec<u8>, ReqwestError> {
    let client = reqwest::Client::new();
    let mut header = HeaderMap::new();

    let random_hex = utils::generate_random_hash(32);
    let random_device = utils::get_random_device();

    let country_code = match cc {
        "jp" => "ja",
        other => other,
    };

    let data = json!({
        "clientInfo": {
            "client": { "countryCode": country_code, "version": version },
            "device": { "model": random_device["model"] },
            "os": { "type": "android", "version": random_device["version"] }
        },
        "nonce": random_hex,
        "pin": password
    });

    header.insert("Content-Type", "application/json".parse().unwrap());
    header.insert("Accept-Encoding", "gzip".parse().unwrap());
    header.insert("Connection", "keep-alive".parse().unwrap());
    header.insert(
        "User-Agent",
        random_device["User-Agent"].to_string().parse().unwrap(),
    );

    let url = format!("https://nyanko-save.ponosgames.com/v2/transfers/{account}/reception");

    let response = client
        .post(url)
        .headers(header)
        .body(data.to_string())
        .send()
        .await?
        .bytes()
        .await?;

    Ok(response.to_vec())
}
