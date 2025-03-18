use bcuzip::file_parser::length_count;
use bcuzip::file_select;

mod bcuzip;
mod functions;

async fn main() {
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
}
