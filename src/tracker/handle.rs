use std::io;

use crate::functions::logger::logger::{log, LogLevel};
use crate::seed::handle::get_seed;
use crate::tracker::event_info_getter::file_getter::{event_info, gatya_info};
use colored::Colorize;

pub async fn get_track() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!("請選擇總子碼輸入方式:\n1. 手動輸入\n2. 從轉移碼和驗證碼獲得");
    io::stdin().read_line(&mut input).expect("讀取失敗");

    let seed = match &input.trim().parse::<u32>() {
        Ok(1) => {
            input.clear();
            println!("請輸入種子碼:");
            io::stdin().read_line(&mut input).expect("讀取失敗");

            input.trim().parse::<u32>().unwrap()
        }
        Ok(2) => get_seed().await?,
        _ => {
            log(LogLevel::Error, format!("Error: Invalid input."));
            return Ok(());
        }
    };

    input.clear();
    println!("請選擇伺服器版本\n1. JP\n2. TW\n3. EN\n4. KR");
    io::stdin().read_line(&mut input).expect("讀取失敗");

    let cc = match input.trim() {
        "1" => "jp",
        "2" => "tw",
        "3" => "en",
        "4" => "kr",
        _ => {
            log(LogLevel::Error, format!("輸入錯誤"));
            return Ok(());
        }
    };

    let (unitbuy_data, gatya_data) = gatya_info(cc).await?;
    if unitbuy_data.is_empty() || gatya_data.is_empty() {
        log(LogLevel::Error, format!("缺少轉蛋資料"));
        return Ok(());
    }

    let event_data = event_info(cc).await?;

    input.clear();
    for (index, event) in event_data.iter().enumerate() {
        match event.force {
            true => println!(
                "{} {}",
                index,
                format!("{} (必中)", event.banner_text).magenta()
            ),
            false => println!("{} {}", index + 1, event.banner_text),
        }
    }

    println!("請選擇欲抽卡池");
    io::stdin().read_line(&mut input).expect("讀取失敗");
    let event_index: u32 = input.trim().parse()?;

    if event_index > 0 && event_index <= event_data.len() as u32 {
        let mut selected = event_data[(event_index - 1) as usize].clone();
        selected.cc = Some(cc.to_string());
        selected.unit_buy = Some(unitbuy_data);
        selected.gatya_data = Some(gatya_data);

        selected.print_track(seed).await?;
    } else {
        log(LogLevel::Error, format!("請輸入正確的值"));
    }

    Ok(())
}
