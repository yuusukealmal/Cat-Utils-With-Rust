use std::io;

use colored::Colorize;

mod bcuzip;
mod event;
mod functions;
mod placement;
mod seed;

#[tokio::main]
async fn main() {
    loop {
        let mut input = String::new();
        println!(
            "請選擇項目: \n1. 獲得活動檔案\n2. 取得公告\n3. 解密bcuzip\n4. 取得種子碼\n5. 退出"
        );
        io::stdin().read_line(&mut input).expect("讀取失敗");
        let number: u32 = input.trim().parse().expect("輸入錯誤");

        match number {
            1 => {
                let _ = event::handle::get_data().await;
                println!("{}", "\n活動檔案下載完成\n".green());
            }
            2 => {
                let _ = placement::handle::get_announcement().await;
                println!("{}", "\n公告下載完成\n".green());
            }
            3 => {
                let _ = bcuzip::handle::decrypt_bcuzip();
                println!("{}", "\n解密完成\n".green());
            }
            4 => {
                let seed = seed::handle::get_seed().await.unwrap();
                println!("{}", format!("\n取得種子碼: {seed}\n").green());
            }
            5 => {
                println!("{}", "\n謝謝使用".green());
                break;
            }
            _ => {
                eprintln!("{}", "\nError: Invalid input.\n".red());
            }
        }
    }
}
