use std::io;

use colored::Colorize;

mod bcuzip;
mod event;
mod functions;
mod local;
mod placement;
mod seed;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    loop {
        let mut input = String::new();
        println!(
            "請選擇項目: \n1. 獲得活動檔案\n2. 取得公告\n3. 拆包apk\n4. 取得伺服器檔案\n5. 解密bcuzip\n6. 取得種子碼\n7. 退出"
        );
        io::stdin().read_line(&mut input).expect("讀取失敗");
        let number: u32 = input.trim().parse().expect("輸入錯誤");

        match number {
            1 => {
                event::handle::get_data().await?;
                println!("{}", "\n活動檔案下載完成\n".green());
            }
            2 => {
                placement::handle::get_announcement().await?;
                println!("{}", "\n公告下載完成\n".green());
            }
            3 => {
                local::handle::dump_apk()?;
                println!("{}", "\n拆包完成\n".green());
            }
            4 => {
                server::handle::get_server_file().await?;
                println!("{}", "\n伺服器檔案下載完成\n".green());
            }
            5 => {
                bcuzip::handle::decrypt_bcuzip()?;
                println!("{}", "\n解密完成\n".green());
            }
            6 => {
                let seed = seed::handle::get_seed().await?;
                println!("{}", format!("\n取得種子碼: {seed}\n").green());
            }
            7 => {
                println!("{}", "\n謝謝使用".green());
                break;
            }
            _ => {
                eprintln!("{}", "\nError: Invalid input.\n".red());
            }
        }
    }

    Ok(())
}
