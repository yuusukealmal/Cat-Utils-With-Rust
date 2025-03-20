use std::io;

use regex::Regex;

use crate::functions::utils::zfill;
use crate::seed::requests;

pub struct SaveParser {
    pub address: usize,
    pub save_data: Vec<u8>,
}

pub async fn get_seed() -> Result<u32, std::io::Error> {
    let account_reg = Regex::new(r"[A-Za-z0-9]{9}").unwrap();
    let mut account = String::new();
    println!("請輸入帳號 (0-9, a-f)");
    io::stdin().read_line(&mut account).expect("讀取失敗");
    if !account_reg.is_match(&account.trim()) {
        println!("帳號格式錯誤");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "帳號格式錯誤"));
    }

    let password_reg = Regex::new(r"[0-9]{4}").unwrap();
    let mut password = String::new();
    println!("請輸入密碼");
    io::stdin().read_line(&mut password).expect("讀取失敗");
    if !password_reg.is_match(&password.trim()) {
        println!("密碼格式錯誤");
        return Err(io::Error::new(io::ErrorKind::InvalidData, "密碼格式錯誤"));
    }

    println!("請選家伺服器版本\n1. JP\n2. TW\n3. EN\n4. KR");
    let mut cc = String::new();
    io::stdin().read_line(&mut cc).expect("讀取失敗");
    let cc = match cc.trim() {
        "1" => "jp",
        "2" => "tw",
        "3" => "en",
        "4" => "kr",
        _ => {
            println!("輸入錯誤");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "輸入錯誤"));
        }
    };

    println!("請輸入版本");
    let mut version = String::new();
    io::stdin().read_line(&mut version).expect("讀取失敗");
    let version = zfill(&version.trim());

    let data = requests::get_save(account.trim(), password.trim(), version, cc)
        .await
        .unwrap();

    let save = SaveParser::new(data).parse_save(None);

    Ok(save)
}
