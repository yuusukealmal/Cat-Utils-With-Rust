use std::io;

use colored::Colorize;

use functions::duration::count_duration;

mod bcuzip;
mod event;
mod functions;
mod local;
mod placement;
mod seed;
mod server;
mod update;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args().collect::<Vec<String>>();
    let launch_time = std::time::Instant::now();
    dotenv::dotenv().ok();

    if arg.len() > 1 && arg[1] == "update" {
        let t = std::time::Instant::now();
        update::handle::update().await?;
        println!(
            "\n{} 花費: {}\n",
            "更新所有檔案完成".green(),
            count_duration(t.elapsed())
        );
    } else {
        loop {
            let mut input = String::new();
            println!(
                "請選擇項目: \n1. 獲得活動檔案\n2. 取得公告\n3. 拆包apk\n4. 取得伺服器檔案\n5. 解密bcuzip\n6. 取得種子碼\n7. 退出"
            );
            io::stdin().read_line(&mut input).expect("讀取失敗");
            let number: u32 = input.trim().parse().expect("輸入錯誤");

            match number {
                1 => {
                    let t = std::time::Instant::now();
                    event::handle::get_event_data(None).await?;
                    println!(
                        "\n{} 花費: {}\n",
                        "活動檔案下載完成".green(),
                        count_duration(t.elapsed())
                    );
                }
                2 => {
                    let t = std::time::Instant::now();
                    placement::handle::get_announcement(None).await?;
                    println!(
                        "\n{} 花費: {}\n",
                        "公告下載完成".green(),
                        count_duration(t.elapsed())
                    );
                }
                3 => {
                    let t = std::time::Instant::now();
                    local::handle::dump_apk(None)?;
                    println!(
                        "\n{} 花費: {}\n",
                        "拆包完成".green(),
                        count_duration(t.elapsed())
                    );
                }
                4 => {
                    let t = std::time::Instant::now();
                    server::handle::get_server_file(None).await?;
                    println!(
                        "\n{} 花費: {}\n",
                        "伺服器檔案下載完成".green(),
                        count_duration(t.elapsed())
                    );
                }
                5 => {
                    let t = std::time::Instant::now();
                    bcuzip::handle::decrypt_bcuzip()?;
                    println!(
                        "\n{} 花費: {}\n",
                        "解密完成".green(),
                        count_duration(t.elapsed())
                    );
                }
                6 => {
                    let t = std::time::Instant::now();
                    let seed = seed::handle::get_seed().await?;
                    println!(
                        "\n取得種子碼: {} 花費: {}\n",
                        format!("{seed}").green(),
                        count_duration(t.elapsed())
                    );
                }
                7 => {
                    println!(
                        "\n{} 使用時間: {}",
                        "謝謝使用".green(),
                        count_duration(launch_time.elapsed())
                    );
                    break;
                }
                _ => {
                    eprintln!("\n{}\n", "Error: Invalid input.".red());
                }
            }
        }
    }

    Ok(())
}
