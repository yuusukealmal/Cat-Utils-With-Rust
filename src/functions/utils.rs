use std::time::{SystemTime, UNIX_EPOCH};

use rand::{self, rng, Rng};

pub fn generate_random_hash(length: usize) -> String {
    let mut rng = rng();
    let chars = "0123456789abcdef";
    let mut res = String::new();

    for _ in 0..length {
        let rand_index = rng.random_range(0..chars.chars().count());
        let random_char = chars.chars().nth(rand_index).unwrap();
        res.push(random_char);
    }

    res
}

pub fn get_timestamp(times: u64) -> String {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * times;

    current_time.to_string()
}
