use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde_json::{json, Value};

pub fn zfill(version: &str) -> u32 {
    version
        .split('.')
        .map(|x| format!("{:02}", x.parse::<u8>().expect("Invalid version part")))
        .collect::<String>()
        .parse()
        .expect("Failed to parse version")
}

pub fn parse_version_int(version: &str) -> Result<u32, String> {
    let mut result = String::new();

    for part in version.split('.') {
        if part.len() > 2 {
            return Err(format!("Error Version Slice: {}", part));
        }
        result.push_str(&format!("{:0>2}", part));
    }

    result
        .parse::<u32>()
        .map_err(|e| format!("Failed to parse version: {}", e))
}

#[allow(dead_code)]
pub fn parse_version_str(version: u64) -> String {
    let version_str = format!(
        "{:0>width$}",
        version,
        width = (version.to_string().len() + 1) / 2 * 2
    );
    version_str
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            let cleaned = s.trim_start_matches('0');
            if cleaned.is_empty() { "0" } else { cleaned }.to_string()
        })
        .collect::<Vec<_>>()
        .join(".")
}

pub fn generate_random_hash(length: usize) -> String {
    let mut rng = rand::rng();
    let chars = b"0123456789abcdef";

    (0..length)
        .map(|_| {
            let rand_index = rng.random_range(0..chars.len());
            chars[rand_index] as char
        })
        .collect()
}

pub fn get_timestamp(multiplier: u64) -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * multiplier
}

pub fn get_random_device() -> Value {
    let devices = vec![
        json!({
            "User-Agent": "Dalvik/2.1.0 (Linux; U; Android 13; LE2110 Build/TKQ1.230329.002)",
            "model": "LE2110",
            "version": "13"
        }),
        json!({
            "User-Agent": "Dalvik/2.1.0 (Linux; U; Android 12; SM-G991B Build/SP1A.210812.016)",
            "model": "SM-G991B",
            "version": "12"
        }),
        json!({
            "User-Agent": "Dalvik/2.1.0 (Linux; U; Android 11; Pixel 5 Build/RQ3A.210805.001.A1)",
            "model": "Pixel 5",
            "version": "11"
        }),
        json!({
            "User-Agent": "Dalvik/2.1.0 (Linux; U; Android 10; MI 9 Build/QKQ1.190825.002)",
            "model": "MI 9",
            "version": "10"
        }),
    ];

    let random_index = rand::rng().random_range(0..devices.len());
    devices[random_index].clone()
}

pub fn get_folder_name(cc: &str) -> String {
    let folder = match cc {
        "jp" => "にゃんこ大戦争",
        "tw" => "貓咪大戰爭",
        "en" => "The Battle Cats",
        "kr" => "냥코대전쟁",
        _ => "Unknown",
    };

    folder.to_string()
}
