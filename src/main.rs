use crate::bcuzip::file_parser::length_count;
use crate::bcuzip::source_selector;

mod bcuzip;

fn main() {
    let file = source_selector::selectfile();
    let dest = source_selector::selectfolder();

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
}
