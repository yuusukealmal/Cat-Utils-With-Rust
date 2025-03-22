use dirs::download_dir;
use rfd::FileDialog;
use std::path::PathBuf;

pub fn file_dialog(
    pick_file: bool,
    name: Option<String>,
    extensions: Option<Vec<&str>>,
) -> Option<PathBuf> {
    let mut dialog = FileDialog::new();

    if let Some(dir) = download_dir() {
        dialog = dialog.set_directory(dir);
    }

    if pick_file {
        if let Some(ext_list) = extensions {
            dialog = dialog.add_filter(
                &name.unwrap_or_else(|| String::from("All Files")),
                &ext_list,
            );
        }
        dialog.pick_file()
    } else {
        dialog.pick_folder()
    }
}
