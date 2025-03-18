use dirs::download_dir;
use rfd::FileDialog;
use std::path::PathBuf;

pub fn file_dialog(
    pick_file: bool,
    name: Option<String>,
    extension: Option<String>,
) -> Option<PathBuf> {
    let mut dialog = FileDialog::new();

    if let Some(dir) = download_dir() {
        dialog = dialog.set_directory(dir);
    }

    if pick_file {
        dialog
            .add_filter(
                name.unwrap_or(String::from("All Files")),
                &[extension.unwrap_or(String::from("*"))],
            )
            .pick_file()
    } else {
        dialog.pick_folder()
    }
}
