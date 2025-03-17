use dirs::download_dir;
use rfd::FileDialog;
use std::path::PathBuf;

pub fn selectfile() -> Option<PathBuf> {
    let file = FileDialog::new()
        .set_directory(download_dir().unwrap())
        .add_filter("BCUZIP files", &["bcuzip"])
        .pick_file();

    file
}

pub fn selectfolder() -> Option<PathBuf> {
    let folder = FileDialog::new()
        .set_directory(download_dir().unwrap())
        .add_filter("BCUZIP files", &["bcuzip"])
        .pick_folder();

    folder
}
