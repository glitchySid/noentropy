use super::types::MoveError;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

pub fn build_target_path(
    base_path: &Path,
    category: &str,
    sub_category: &str,
    filename: &str,
) -> PathBuf {
    let mut final_path = base_path.join(category);
    if !sub_category.is_empty() {
        final_path = final_path.join(sub_category);
    }

    let file_name = Path::new(filename)
        .file_name()
        .unwrap_or_else(|| OsStr::new(filename))
        .to_string_lossy()
        .into_owned();

    final_path.join(&file_name)
}

pub fn ensure_directory_exists(path: &Path) -> Result<(), MoveError> {
    fs::create_dir_all(path).map_err(|e| MoveError::DirectoryCreationFailed(path.to_path_buf(), e))
}
