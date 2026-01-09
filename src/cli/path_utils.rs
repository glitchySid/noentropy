use std::path::{Path, PathBuf};

/// Validates that a path exists and is a readable directory.
/// Returns the canonicalized path if validation succeeds.
pub async fn validate_and_normalize_path(path: &Path) -> Result<PathBuf, String> {
    // Use tokio::fs for async file operations
    let metadata = tokio::fs::metadata(path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            format!("Path '{}' does not exist", path.display())
        } else {
            format!("Cannot access '{}': {}", path.display(), e)
        }
    })?;

    if !metadata.is_dir() {
        return Err(format!("Path '{}' is not a directory", path.display()));
    }

    // Check if we can read the directory
    let _ = tokio::fs::read_dir(path)
        .await
        .map_err(|e| format!("Cannot access directory '{}': {}", path.display(), e))?;

    // canonicalize is sync-only, use spawn_blocking
    let path_owned = path.to_path_buf();
    tokio::task::spawn_blocking(move || path_owned.canonicalize())
        .await
        .map_err(|e| format!("Task failed: {}", e))?
        .map_err(|e| format!("Failed to normalize path '{}': {}", path.display(), e))
}
