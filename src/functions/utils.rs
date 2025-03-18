use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

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
