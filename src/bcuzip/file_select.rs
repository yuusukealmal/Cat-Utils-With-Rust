use std::path::PathBuf;

use crate::functions::file_selector::file_dialog;

pub fn selectfile() -> Option<PathBuf> {
    file_dialog(
        true,
        Some("BCUZIP files".to_string()),
        Some(["bcuzip"].to_vec()),
    )
}

pub fn selectfolder() -> Option<PathBuf> {
    file_dialog(false, None, None)
}
