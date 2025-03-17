use dirs::download_dir;
use rfd::FileDialog;
use std::path::PathBuf;

fn file_dialog(pick_file: bool) -> Option<PathBuf> {
    let mut dialog = FileDialog::new();

    if let Some(dir) = download_dir() {
        dialog = dialog.set_directory(dir);
    }

    if pick_file {
        dialog.add_filter("BCUZIP files", &["bcuzip"]).pick_file()
    } else {
        dialog.pick_folder()
    }
}

pub fn selectfile() -> Option<PathBuf> {
    file_dialog(true)
}

pub fn selectfolder() -> Option<PathBuf> {
    file_dialog(false)
}
