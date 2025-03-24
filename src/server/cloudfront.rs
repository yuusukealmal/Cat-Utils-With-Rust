use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePrivateKey, Pkcs1v15Sign, RsaPrivateKey};
use serde_json::json;
use sha1::{Digest, Sha1};
use std::error::Error;

pub struct CloudFrontSign {
    cf_private_key: String,
    cf_key_pair_id: String,
}

impl CloudFrontSign {
    pub fn new() -> Self {
        let cf_private_key = "-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQCORX64nic2atwz
1VsqbI/jqHwrdrSktgjoBrFWqgziJXrVvHVZ+shhUfRxa4BKvdRigbZuNjrmFfUE
Scxdfj72QAe0SRxgMhaloPikbSUlvrfOacFOiQD7dMtwv8DAAjvshKU/qmzdkp1j
DC3QcIX5gsuNqUuM6SxPBZviuvxD/IsbLlxxUCh30hZNUb/3aTf00SzI1/rAL2jB
faQfR6FY2qt0yICoXaVwCu6GObJDPdMjX8ssCmTxRb81aAnfUy6A9x/ywutYjjaY
W2RA9hzWCxk6nONpuRQsZkEbNULoKzgyfxMc/xNfFYH+0T2ui59zsRVTNsCHYVEb
iz9an3HXAgMBAAECggEAGMPgGyLskHxpeFxbUjczlN1vP+GZ8FH/muQPWpafR35e
s3Xqt47/8nDhrByaaGhC4CLULrsh5YtM60ItYNjo/NSIgsl3NweBCbPLlFOrc7aP
KE8gZxtSIHNkNmwqkUHSTImKelqgOLGc0/D6yJ3NtHEgHbiqfgzYuaiwSfdikjLR
T5sRs7T5k0Gy67FSOOa43s4WHj77ywdcvYbzBdSM/uu8R2Syng4RijKCvKLIEIE0
3lPDI9/KNG8ofSyaqjLa09xfooQ7S8La21El3iu/icOowR0WM+hXLcEilCkuMFY1
IrheIsx2Pyb6N3qwE/jRMIqQwH5uzM8OThmU+17tgQKBgQDCwrR33GXg5SNvSg4N
iBsuV1p2sXeuJxZrSCyNUVaT7lg01dUnk9MbAflqNsD43mHdXWYLd8FqKZ4Fc3K/
t3sdYJPeOmMIKmBexhOnohoAwg4FZhuUvzhx6DPuBG2gSCzzGNjU9SXCr9TQlwkc
XTRIEjBe5JuFFhHGe+xMVEu+rQKBgQC7AasNilpc7lhXuNi/fXdJwqX+s3Jiolys
8BQJ83i1Xy7pzfa2usOjYLFcebcXv9lTfrDUW4ju1ip8zVVWkdF9xE9uRFDbSBJo
MvQNa9bGgFlLu/V5XsbCFbFrYqI94OHIT4/2dHJsyeJpoXpoFHZ2aGt/98fFEGqU
YbAdR/HXEwKBgQCPMEUshm6knPKjZKfWTQXm2TRaZXmfIX+7GlIfB/kGQ8q39aqE
MYuYpKfx7hWMIzuCW6OltMMPwU87pLhtuYEbhSDR1s1ueHFn3Gsg6O4DNqjGUV7f
yoK+REDBsqHCoK3jgJYSY7YCX/Gv9gstvlyszCqh6aNpgmNJMVz2dVdG9QKBgQCK
G2FITrUNjLiRkGICiZZfUvFkeQIw9deboHIsJzMuP21WHlXl/WgecHqL4Rfm4jiO
ATJ2omMuf9xA7yPnGymryB8hQDK2vzNY4Mh8YPftATzxQY64Y9ZF3993fxBywnH8
jUW0rasTzMT5XdgYpYQXTmaVy1gtoUIU81AtT8S7IQKBgQC2F7xdWSv7Pw+MimN2
Tx8VMiCUkL+5uNJwvWw2rrEHvt2jphD016pgdutlgI28qoXwcleLxAz1Ey1njCTO
19bsOA9bhuwbIrIb93nGHyRrQe1L7PdBjwlIqEj8R08Z/oGQsXhqzgF9KfO2V46i
oPSxLzYw2sBjmwVooXMVr6GxEw==
-----END PRIVATE KEY-----".to_string();

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
    }
    

    fn make_signature(&self, message: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(&self.cf_private_key)?;
        // 注意：此處需與 Python 一致，直接簽署原始消息（無需顯式哈希）
        let signature = private_key.sign(
            Pkcs1v15Sign::new_unprefixed(),
            message.as_bytes()
        )?;
        Ok(signature)
    }

    fn generate_signature(&self, policy: &str) -> Result<String, Box<dyn Error>> {
        let signature = self.make_signature(policy)?;
        Ok(general_purpose::STANDARD.encode(signature))
    }

    pub fn generate_signed_cookie(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let policy = self.make_policy(url);
        let signature = self.generate_signature(&policy)?;
        let encoded_policy = general_purpose::STANDARD.encode(policy.as_bytes());

        Ok(format!(
            "CloudFront-Key-Pair-Id={}; CloudFront-Policy={}; CloudFront-Signature={}",
            self.cf_key_pair_id, encoded_policy, signature
        ))
    }
}
