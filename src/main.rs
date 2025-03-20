use std::io;

use bcuzip::file_parser::length_count;
use bcuzip::file_select;

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
            "請選擇項目: \n1. 獲得活動檔案\n2. 解密bcuzip\n3. 取得公告\n4. 取得種子碼\n5. 退出"
        );
        io::stdin().read_line(&mut input).expect("讀取失敗");
        let number: u32 = input.trim().parse().expect("輸入錯誤");

        match number {
            1 => {
                event::handle::get_data().await;
                println!("活動檔案下載完成");
            }
            2 => {
                let file = file_select::selectfile();
                let dest = file_select::selectfolder();

                match (file, dest) {
                    (Some(file_path), Some(dest_path)) => {
                        let file_str = file_path.to_string_lossy();
                        let dest_str = dest_path.to_string_lossy();
                        let _ = length_count::parse_file(&file_str, &dest_str);
                    }
                    _ => {
                        eprintln!("Error: No file or destination folder selected.");
                        return;
                    }
                }

                println!("解密完成");
            }
            3 => {
                let _ = placement::handle::get_announcement().await;
                println!("公告下載完成");
            }
            4 => {
                let seed = seed::handle::get_seed().await;
                println!("取得種子碼: {}", seed.unwrap());
            }
            5 => {
                println!("謝謝使用");
                break;
            }
            _ => {
                eprintln!("Error: Invalid input.");
            }
        }
    }
}
