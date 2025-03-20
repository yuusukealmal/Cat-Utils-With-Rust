use rand::Rng;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn zfill(version: &str) -> u32 {
    let mut res = String::new();
    let v = version.split(".");

    for i in v {
        if i.len() == 1 {
            res.push('0');
        }
        res.push_str(i);
    }

    res.parse().unwrap()
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

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..devices.len());
    devices[random_index].clone()
}
