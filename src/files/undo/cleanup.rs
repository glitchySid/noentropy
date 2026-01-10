use crate::storage::UndoLog;
use colored::*;
use std::fs;
use std::path::Path;

pub(super) fn cleanup_empty_directories(
    base_path: &Path,
    undo_log: &mut UndoLog,
) -> Result<(), Box<dyn std::error::Error>> {
    let directory_usage = undo_log.get_directory_usage(base_path);

    for dir_path in directory_usage.keys() {
        let full_path = base_path.join(dir_path);
        if full_path.is_dir()
            && let Ok(mut entries) = fs::read_dir(&full_path)
            && entries.next().is_none()
            && fs::remove_dir(&full_path).is_ok()
        {
            println!("{} Removed empty directory: {}", "INFO:".cyan(), dir_path);
        }
    }

    Ok(())
}
