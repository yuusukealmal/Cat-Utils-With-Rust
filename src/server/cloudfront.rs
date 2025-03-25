use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose;
use base64::Engine;
use rsa::{RsaPrivateKey, pkcs1v15::Pkcs1v15Sign}; // 直接使用 pkcs1v15 模組
use sha1::{Sha1, Digest};
use pkcs8::DecodePrivateKey;
use hex;

pub struct CloudFrontSign {
    cf_private_key: String,
    cf_key_pair_id: String,
}

impl CloudFrontSign {
    pub fn new() -> Self {
        let cf_private_key = "-----BEGIN PRIVATE KEY-----\nMIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQCORX64nic2atwz\n1VsqbI/jqHwrdrSktgjoBrFWqgziJXrVvHVZ+shhUfRxa4BKvdRigbZuNjrmFfUE\nScxdfj72QAe0SRxgMhaloPikbSUlvrfOacFOiQD7dMtwv8DAAjvshKU/qmzdkp1j\nDC3QcIX5gsuNqUuM6SxPBZviuvxD/IsbLlxxUCh30hZNUb/3aTf00SzI1/rAL2jB\nfaQfR6FY2qt0yICoXaVwCu6GObJDPdMjX8ssCmTxRb81aAnfUy6A9x/ywutYjjaY\nW2RA9hzWCxk6nONpuRQsZkEbNULoKzgyfxMc/xNfFYH+0T2ui59zsRVTNsCHYVEb\niz9an3HXAgMBAAECggEAGMPgGyLskHxpeFxbUjczlN1vP+GZ8FH/muQPWpafR35e\ns3Xqt47/8nDhrByaaGhC4CLULrsh5YtM60ItYNjo/NSIgsl3NweBCbPLlFOrc7aP\nKE8gZxtSIHNkNmwqkUHSTImKelqgOLGc0/D6yJ3NtHEgHbiqfgzYuaiwSfdikjLR\nT5sRs7T5k0Gy67FSOOa43s4WHj77ywdcvYbzBdSM/uu8R2Syng4RijKCvKLIEIE0\n3lPDI9/KNG8ofSyaqjLa09xfooQ7S8La21El3iu/icOowR0WM+hXLcEilCkuMFY1\nIrheIsx2Pyb6N3qwE/jRMIqQwH5uzM8OThmU+17tgQKBgQDCwrR33GXg5SNvSg4N\niBsuV1p2sXeuJxZrSCyNUVaT7lg01dUnk9MbAflqNsD43mHdXWYLd8FqKZ4Fc3K/\nt3sdYJPeOmMIKmBexhOnohoAwg4FZhuUvzhx6DPuBG2gSCzzGNjU9SXCr9TQlwkc\nXTRIEjBe5JuFFhHGe+xMVEu+rQKBgQC7AasNilpc7lhXuNi/fXdJwqX+s3Jiolys\n8BQJ83i1Xy7pzfa2usOjYLFcebcXv9lTfrDUW4ju1ip8zVVWkdF9xE9uRFDbSBJo\nMvQNa9bGgFlLu/V5XsbCFbFrYqI94OHIT4/2dHJsyeJpoXpoFHZ2aGt/98fFEGqU\nYbAdR/HXEwKBgQCPMEUshm6knPKjZKfWTQXm2TRaZXmfIX+7GlIfB/kGQ8q39aqE\nMYuYpKfx7hWMIzuCW6OltMMPwU87pLhtuYEbhSDR1s1ueHFn3Gsg6O4DNqjGUV7f\nyoK+REDBsqHCoK3jgJYSY7YCX/Gv9gstvlyszCqh6aNpgmNJMVz2dVdG9QKBgQCK\nG2FITrUNjLiRkGICiZZfUvFkeQIw9deboHIsJzMuP21WHlXl/WgecHqL4Rfm4jiO\nATJ2omMuf9xA7yPnGymryB8hQDK2vzNY4Mh8YPftATzxQY64Y9ZF3993fxBywnH8\njUW0rasTzMT5XdgYpYQXTmaVy1gtoUIU81AtT8S7IQKBgQC2F7xdWSv7Pw+MimN2\nTx8VMiCUkL+5uNJwvWw2rrEHvt2jphD016pgdutlgI28qoXwcleLxAz1Ey1njCTO\n19bsOA9bhuwbIrIb93nGHyRrQe1L7PdBjwlIqEj8R08Z/oGQsXhqzgF9KfO2V46i\noPSxLzYw2sBjmwVooXMVr6GxEw==\n-----END PRIVATE KEY-----".to_string();

        let cf_key_pair_id = "APKAJO6MLYTURWB2NOWQ".to_string();

        Self {
            cf_private_key,
            cf_key_pair_id,
        }
    }

    fn make_policy(&self, url: &str) -> String {
        let fixed_timestamp = 1710000000; // 固定時間戳（示例值）
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
            fixed_timestamp + 60 * 60,
            fixed_timestamp - 60 * 60
        )
        .replace(" ", "")
        .replace("\n", "")
    }

    fn make_signature(&self, message: &str) -> Result<Vec<u8>, Box<dyn Error>> {        
        let private_key = RsaPrivateKey::from_pkcs8_pem(&self.cf_private_key)?;
        let message_bytes = message.as_bytes();
        println!("message: {}", hex::encode(message_bytes));
        
        let mut hasher = Sha1::new();
        hasher.update(message_bytes);
        let hashed = hasher.finalize();
        // println!("hash: {}", hex::encode(&hashed));
        
        let signature = private_key.sign(Pkcs1v15Sign::new_unprefixed(), &hashed)?;
        // println!("hex: {}", hex::encode(&signature));
        
        Ok(signature)
    }

    pub fn generate_signed_cookie(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let policy = self.make_policy(url);
        // println!("{}", policy);
        let signature = self.make_signature(&policy)?;
        let encoded_policy = general_purpose::STANDARD.encode(&policy);
        let encoded_signature = general_purpose::STANDARD.encode(&signature);
        // println!("base64: {}", encoded_signature);
        Ok(format!(
            "CloudFront-Key-Pair-Id={};\n\nCloudFront-Policy={};\n\nCloudFront-Signature={}",
            self.cf_key_pair_id, encoded_policy, encoded_signature
        ))
    }
}
