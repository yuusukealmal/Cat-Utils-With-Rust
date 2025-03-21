use colored::Colorize;

use crate::functions::logger::logger::{log, LogLevel};
use crate::bcuzip::file_select;
use crate::bcuzip::file_parser::length_count;

pub fn decrypt_bcuzip() {
    let file = file_select::selectfile();
    let dest = file_select::selectfolder();

    match (file, dest) {
        (Some(file_path), Some(dest_path)) => {
            let file_str = file_path.to_string_lossy();
            log(LogLevel::Info, format!("Selected file: {}", file_str));
            let dest_str = dest_path.to_string_lossy();
            log(
                LogLevel::Info,
                format!("Selected destination folder: {}", dest_str),
            );
            let _ = length_count::parse_file(&file_str, &dest_str);
        }
        _ => {
            eprintln!("{}", "Error: No file or destination folder selected.".red());
            return;
        }
    }
}
