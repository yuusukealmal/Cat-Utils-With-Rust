use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose;
use base64::Engine;
use pkcs8::DecodePrivateKey;
use rsa::{pkcs1v15::Pkcs1v15Sign, RsaPrivateKey}; // 直接使用 pkcs1v15 模組
use sha1::{Digest, Sha1};

pub struct CloudFrontSign {
    cf_private_key: String,
    cf_key_pair_id: String,
}

impl CloudFrontSign {
    pub fn new() -> Self {
        let cf_private_key = fs::read_to_string("cf_private_key.pem").unwrap();

        let cf_key_pair_id = "APKAJO6MLYTURWB2NOWQ".to_string();

        Self {
            cf_private_key,
            cf_key_pair_id,
        }
    }

    fn make_policy(&self, url: &str) -> String {
        format!(
            r#"{{
                "Statement": [{{
                    "Resource": "{}",
                    "Condition": {{
                        "DateLessThan": {{"AWS:EpochTime": {}}},
                        "DateGreaterThan": {{"AWS:EpochTime": {}}}
                    }}
                }}]
            }}"#,
            url,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 60 * 60,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                - 60 * 60
        )
        .replace(" ", "")
        .replace("\n", "")
    }

    fn make_signature(&self, message: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        // Load the private key from a PEM string
        let private_key = RsaPrivateKey::from_pkcs8_pem(&self.cf_private_key)?;
        let message_bytes = message.as_bytes();

        // Create a SHA-1 hasher and hash the message
        let mut hasher = Sha1::new();
        hasher.update(message_bytes);
        let hashed = hasher.finalize();

        // Sign the hashed message using PKCS#1 v1.5 with SHA-1
        let signature = private_key.sign(Pkcs1v15Sign::new::<Sha1>(), &hashed)?;

        Ok(signature)
    }
    pub fn generate_signed_cookie(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let policy = self.make_policy(url);
        let signature = self.make_signature(&policy)?;

        let encoded_policy = general_purpose::STANDARD.encode(&policy);
        let encoded_signature = general_purpose::STANDARD.encode(&signature);

        Ok(format!(
            "CloudFront-Key-Pair-Id={}; CloudFront-Policy={}; CloudFront-Signature={}",
            self.cf_key_pair_id, encoded_policy, encoded_signature
        ))
    }
}
