use std::io;

use colored::Colorize;
use regex::Regex;

use crate::functions::logger::logger::{log, LogLevel};
use crate::functions::utils::zfill;

pub struct Account {
    pub account: String,
    pub password: String,
    pub cc: String,
    pub version: u32,
}

pub struct SaveParser {
    pub address: usize,
    pub save_data: Vec<u8>,
}

pub async fn get_seed() -> Result<u32, std::io::Error> {
    let account_reg = Regex::new(r"^[A-Za-z0-9]{9}$").expect("Regex 建立失敗");
    let password_reg = Regex::new(r"^[0-9]{4}$").expect("Regex 建立失敗");

    let mut input = String::new();

    println!("請輸入轉移碼 (0-9, a-f)");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let account = input.trim().to_string();

    if !account_reg.is_match(&account) {
        println!("{}", "轉移碼格式錯誤".red());
        return Err(io::Error::new(io::ErrorKind::InvalidData, "轉移碼格式錯誤"));
    }
    log(LogLevel::Info, format!("Account: {}", account));

    input.clear();
    println!("請輸入驗證碼");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let password = input.trim().to_string();

    if !password_reg.is_match(&password) {
        println!("{}", "驗證碼格式錯誤".red());
        return Err(io::Error::new(io::ErrorKind::InvalidData, "驗證碼格式錯誤"));
    }
    log(LogLevel::Info, format!("Password: {}", password));

    input.clear();
    println!("請選擇伺服器版本\n1. JP\n2. TW\n3. EN\n4. KR");
    io::stdin().read_line(&mut input).expect("讀取失敗");

    let cc = match input.trim() {
        "1" => "jp",
        "2" => "tw",
        "3" => "en",
        "4" => "kr",
        _ => {
            println!("{}", "輸入錯誤".red());
            return Err(io::Error::new(io::ErrorKind::InvalidData, "輸入錯誤"));
        }
    };
    log(LogLevel::Info, format!("Server: {}", cc));

    input.clear();
    println!("請輸入版本");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let version = zfill(input.trim());
    log(LogLevel::Info, format!("Version: {}", version));

    let acc = Account {
        account,
        password,
        cc: cc.to_string(),
        version,
    };

    let data = acc
        .get_save()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let seed = SaveParser::new(data).parse_save(None);

    Ok(seed)
}
