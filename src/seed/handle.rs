use std::io;

use regex::Regex;

use crate::functions::utils::zfill;
use crate::seed::requests;

pub struct SaveParser {
    pub address: usize,
    pub save_data: Vec<u8>,
}

pub async fn get_seed() -> Result<u32, std::io::Error> {
    let account_reg = Regex::new(r"^[A-Za-z0-9]{9}$").expect("Regex 建立失敗");
    let password_reg = Regex::new(r"^[0-9]{4}$").expect("Regex 建立失敗");

    let mut input = String::new();

    println!("請輸入帳號 (0-9, a-f)");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let account = input.trim().to_string();

    if !account_reg.is_match(&account) {
        println!("帳號格式錯誤");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "帳號格式錯誤"));
    }

    input.clear();
    println!("請輸入密碼");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let password = input.trim().to_string();

    if !password_reg.is_match(&password) {
        println!("密碼格式錯誤");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "密碼格式錯誤"));
    }

    input.clear();
    println!("請選擇伺服器版本\n1. JP\n2. TW\n3. EN\n4. KR");
    io::stdin().read_line(&mut input).expect("讀取失敗");

    let cc = match input.trim() {
        "1" => "jp",
        "2" => "tw",
        "3" => "en",
        "4" => "kr",
        _ => {
            println!("輸入錯誤");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "輸入錯誤"));
        }
    };

    input.clear();
    println!("請輸入版本");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let version = zfill(input.trim());

    let data = requests::get_save(&account, &password, version, cc).await;

    let seed = SaveParser::new(data.unwrap()).parse_save(None);

    Ok(seed)
}
