use hex;
use hmac::{Hmac, Mac};
use reqwest::{header::HeaderMap, Response};
use serde_json::json;
use sha2::Sha256;

use crate::functions::utils::{generate_random_hash, get_timestamp};
use crate::config::structs::Event;

impl Event {
    pub fn new() -> Event {
        Event {
            account_code: None,
            password: None,
            password_refresh_token: None,
            jwt_token: None,
            output_path: None,
        }
    }

    fn hmac_sha256(&self, key: &str, message: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("Invalid key length");
        mac.update(message.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    fn get_nyanko_signature(&self, json_text: &str) -> Option<String> {
        let random_hex = generate_random_hash(64);
        let key = self
            .account_code
            .as_deref()
            .map(|acc| format!("{acc}{random_hex}"))?;
        Some(format!("{random_hex}{}", self.hmac_sha256(&key, json_text)))
    }

    async fn get_post_response(&self, url: &str, json_text: String) -> reqwest::Result<Response> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();

        let signature = self.get_nyanko_signature(&json_text).unwrap();
        headers.insert("Nyanko-Signature", signature.parse().unwrap());
        headers.insert("Nyanko-Signature-Version", "1".parse().unwrap());
        headers.insert("Nyanko-Signature-Algorithm", "HMACSHA256".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert(
            "Nyanko-Timestamp",
            get_timestamp(1000).to_string().parse().unwrap(),
        );
        headers.insert(
            "User-Agent",
            "Dalvik/2.1.0 (Linux; U; Android 13; XQ-BC52 Build/61.2.A.0.447)"
                .parse()
                .unwrap(),
        );
        headers.insert("Connection", "Keep-Alive".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip".parse().unwrap());

        client
            .post(url)
            .headers(headers)
            .body(json_text)
            .send()
            .await
    }

    pub async fn generate_account(&mut self) -> reqwest::Result<()> {
        let account_json: serde_json::Value = reqwest::get(
            "https://nyanko-backups.ponosgames.com/?action=createAccount&referenceId=",
        )
        .await?
        .json()
        .await?;

        if account_json["success"].as_bool().unwrap_or(false) {
            self.account_code = account_json["accountId"].as_str().map(String::from);

            let time = get_timestamp(1);
            let random_hex = generate_random_hash(32);

            let password_headers_data = json!({
                "accountCode": self.account_code.as_deref(),
                "accountCreatedAt": time,
                "nonce": random_hex,
            });

            let json_text = password_headers_data.to_string();

            let password_result = self
                .get_post_response("https://nyanko-auth.ponosgames.com/v1/users", json_text)
                .await?
                .text()
                .await?;

            let password_json: serde_json::Value = serde_json::from_str(&password_result).unwrap();

            if password_json["statusCode"] == 1 {
                self.password = password_json["payload"]["password"]
                    .as_str()
                    .map(String::from);

                self.password_refresh_token = password_json["payload"]["passwordRefreshToken"]
                    .as_str()
                    .map(String::from);
            }
        }
        Ok(())
    }

    pub async fn generate_jwtoken(&mut self) -> reqwest::Result<()> {
        if self.account_code.is_none() || self.password.is_none() {
            return Ok(());
        }

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
            .await?
            .text()
            .await?;

        let jwt_json: serde_json::Value = serde_json::from_str(&jwt_result.as_str()).unwrap();

        if jwt_json["statusCode"] == 1 {
            self.jwt_token = jwt_json["payload"]["token"].as_str().map(String::from);
        }
        Ok(())
    }
}
