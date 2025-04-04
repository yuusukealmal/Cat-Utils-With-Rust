use reqwest::header::HeaderMap;
use reqwest::Error as ReqwestError;
use serde_json::json;

use crate::config::routes::GET_SAVE_FILE;
use crate::config::structs::Account;
use crate::functions::utils;

impl Account {
    pub async fn get_save(&self) -> Result<Vec<u8>, ReqwestError> {
        let client = reqwest::Client::new();
        let mut header = HeaderMap::new();

        let random_hex = utils::generate_random_hash(32);
        let random_device = utils::get_random_device();

        let country_code = match self.cc.as_str() {
            "jp" => "ja",
            other => other,
        };

        let data = json!({
            "clientInfo": {
                "client": { "countryCode": country_code, "version": self.version },
                "device": { "model": random_device["model"] },
                "os": { "type": "android", "version": random_device["version"] }
            },
            "nonce": random_hex,
            "pin": self.password
        });

        header.insert("Content-Type", "application/json".parse().unwrap());
        header.insert("Accept-Encoding", "gzip".parse().unwrap());
        header.insert("Connection", "keep-alive".parse().unwrap());
        header.insert(
            "User-Agent",
            random_device["User-Agent"].to_string().parse().unwrap(),
        );

        let url = GET_SAVE_FILE(&self.account);

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
}
