use std::collections::HashMap;

use hex;
use hmac::{Hmac, Mac};
use reqwest::{self, header::HeaderMap, Response};
use serde_json::json;
use sha2::Sha256;

use crate::functions::utils::{generate_random_hash, get_timestamp};
use crate::functions::writer::create_file;

#[derive(Debug)]
pub struct EventData {
    pub cc: Option<String>,
    pub account_code: Option<String>,
    pub password: Option<String>,
    pub password_refresh_token: Option<String>,
    pub jwt_token: Option<String>,
}

impl EventData {
    fn hmac_sha256(&self, key: String, message: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
        mac.update(message.as_bytes());
        let result = mac.finalize();

        hex::encode(result.into_bytes())
    }

    fn get_nyanko_signature(&self, json_text: &str) -> String {
        let random_hex = generate_random_hash(64);
        let key = format!("{}{}", self.account_code.as_ref().unwrap(), random_hex);

        let hmac_result = self.hmac_sha256(key, json_text);

        let signature = format!("{}{}", random_hex, hmac_result);
        signature
    }

    async fn get_post_response(&self, url: &str, json_text: String) -> Response {
        let client = reqwest::Client::new();
        let mut headers: HeaderMap = HeaderMap::new();

        headers.insert(
            "Nyanko-Signature",
            self.get_nyanko_signature(&json_text).parse().unwrap(),
        );
        headers.insert("Nyanko-Signature-Version", "1".parse().unwrap());
        headers.insert("Nyanko-Signature-Algorithm", "HMACSHA256".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Nyanko-Timestamp", get_timestamp(100).parse().unwrap());
        headers.insert(
            "User-Agent",
            "Dalvik/2.1.0 (Linux; U; Android 13; XQ-BC52 Build/61.2.A.0.447)"
                .parse()
                .unwrap(),
        );
        headers.insert("Connection", "Keep-Alive".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip".parse().unwrap());

        let response = client
            .post(url)
            .headers(headers)
            .body(json_text)
            .send()
            .await
            .unwrap();

        response
    }

    async fn generate_account(&mut self) {
        let acc = reqwest::get(
            "https://nyanko-backups.ponosgames.com/?action=createAccount&referenceId=",
        )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        let account_json: serde_json::Value = serde_json::from_str(&acc).unwrap();

        if account_json["success"].as_bool().unwrap() {
            self.account_code = Some(account_json["accountId"].as_str().unwrap().to_string());
            let time = get_timestamp(1);
            let random_hex = generate_random_hash(32);

            let mut password_headers_data: HashMap<String, String> = HashMap::new();
            password_headers_data.insert(
                "accountCode".to_string(),
                self.account_code.clone().unwrap(),
            );
            password_headers_data.insert("accountCreatedAt".to_string(), time);
            password_headers_data.insert("nonce".to_string(), random_hex);

            let json_text = serde_json::to_string(&password_headers_data).unwrap();

            let password_result = self
                .get_post_response("https://nyanko-auth.ponosgames.com/v1/users", json_text)
                .await
                .text()
                .await
                .unwrap();

            let password_json: serde_json::Value = serde_json::from_str(&password_result).unwrap();

            if password_json["statusCode"] == 1 {
                self.password = Some(
                    password_json["payload"]["password"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                );
                self.password_refresh_token = Some(
                    password_json["payload"]["passwordRefreshToken"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                );
            }
        }
    }

    async fn generate_jwtoken(&mut self) {
        if self.account_code.is_some() && self.password.is_some() {
            let json_text = json!({
                "accountCode": self.account_code,
                "clientInfo": {
                    "client": {
                        "countryCode": "ja",
                        "version": "999999"
                    },
                    "device": {
                        "model": "XQ-BC52"
                    },
                    "os": {
                        "type": "android",
                        "version": "Android 13"
                    }
                },
                "nonce": generate_random_hash(32),
                "password": self.password
            });

            let jwt_result = self
                .get_post_response(
                    "https://nyanko-auth.ponosgames.com/v1/tokens",
                    json_text.to_string(),
                )
                .await
                .text()
                .await
                .unwrap();

            let jwt_json: serde_json::Value = serde_json::from_str(&jwt_result).unwrap();

            if jwt_json["statusCode"] == 1 {
                self.jwt_token = Some(jwt_json["payload"]["token"].as_str().unwrap().to_string());
            }
        }
    }

    pub async fn to_file(
        &mut self,
        mut output_path: String,
        mut cc: &str,
        file: &str,
    ) -> Result<(), std::io::Error> {
        self.generate_account().await;
        self.generate_jwtoken().await;

        if cc == "jp" {
            cc = "";
        }

        let url = format!(
            "https://nyanko-events.ponosgames.com/battlecats{cc}_production/{file}.tsv?jwt={}",
            self.jwt_token.as_ref().unwrap()
        );

        let data = reqwest::get(url).await.unwrap().text().await.unwrap();
        output_path.push_str(format!("\\{}_{}.tsv", self.cc.as_ref().unwrap().to_uppercase(), file).as_str());
        create_file(data.as_bytes(), &output_path)?;

        Ok(())
    }
}
