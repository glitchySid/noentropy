//! Unit tests for handle_undo handler
//!
//! Tests the undo functionality including:
//! - No undo log exists
//! - No completed moves to undo
//! - Path validation
//! - Dry run behavior
//! - Successful undo operations

use noentropy::cli::args::Command;
use noentropy::cli::handlers::handle_undo;
use noentropy::cli::path_utils::validate_and_normalize_path;
use noentropy::storage::UndoLog;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Helper to create test Command::Undo
fn create_test_undo_command(dry_run: bool, path: Option<PathBuf>) -> Command {
    Command::Undo { dry_run, path }
}

/// Helper to setup a temp directory with files and subdirectories for undo testing
fn setup_test_dir_for_undo() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Create source directory with files
    let images_dir = dir_path.join("Images");
    fs::create_dir_all(&images_dir).unwrap();

    // Create files that were "moved" to the Images directory
    let photo1 = images_dir.join("photo1.jpg");
    let photo2 = images_dir.join("photo2.png");
    File::create(&photo1).unwrap();
    File::create(&photo2).unwrap();

    // Create source locations that no longer exist (after move)
    let source1 = dir_path.join("photo1.jpg");
    let source2 = dir_path.join("photo2.png");

    (temp_dir, dir_path)
}

/// Helper to create an undo log with completed moves
fn create_undo_log_with_moves(undo_log_path: &Path, moves: Vec<(PathBuf, PathBuf)>) {
    let mut undo_log = UndoLog::new();

    for (source, dest) in moves {
        undo_log.record_move(source, dest);
    }

    undo_log.save(undo_log_path).unwrap();
}

// ============================================================================
// HANDLE_UNDO TESTS
// ============================================================================

#[tokio::test]
async fn test_handle_undo_no_undo_log_exists() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    let command = create_test_undo_command(false, None);

    // Don't create an undo log file - it should handle gracefully
    let result = handle_undo(&command, dir_path).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_undo_no_completed_moves() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Create an empty undo log (no completed moves)
    let undo_log_path = dir_path.join("undo_log.json");
    let undo_log = UndoLog::new();
    undo_log.save(&undo_log_path).unwrap();

    let command = create_test_undo_command(false, None);
    let result = handle_undo(&command, dir_path).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_undo_with_custom_path() {
    let temp_dir = TempDir::new().unwrap();
    let custom_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();
    let custom_path = custom_dir.path().to_path_buf();

    // Create undo log in the main temp dir
    let undo_log_path = dir_path.join("undo_log.json");

    // Create source files in custom path
    let source_file = custom_path.join("test.txt");
    File::create(&source_file).unwrap();

    // Create destination (Images directory)
    let images_dir = custom_path.join("Images");
    fs::create_dir_all(&images_dir).unwrap();
    let dest_file = images_dir.join("test.txt");
    File::create(&dest_file).unwrap();

    // Create undo log with a move
    create_undo_log_with_moves(&undo_log_path, vec![(source_file.clone(), dest_file)]);

    // Use custom path argument
    let command = create_test_undo_command(true, Some(custom_path.clone()));
    let result = handle_undo(&command, custom_path).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_undo_dry_run_no_changes() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Setup directory with files
    let images_dir = dir_path.join("Images");
    fs::create_dir_all(&images_dir).unwrap();
    let photo = images_dir.join("photo.jpg");
    let source = dir_path.join("photo.jpg");
    File::create(&photo).unwrap();

    // Create undo log
    let undo_log_path = dir_path.join("undo_log.json");
    create_undo_log_with_moves(&undo_log_path, vec![(source.clone(), photo.clone())]);

    // Dry run should not actually undo
    let command = create_test_undo_command(true, None);
    let result = handle_undo(&command, dir_path).await;

    assert!(result.is_ok());
    // File should still be in Images directory (dry run)
    assert!(photo.exists());
    // Source should still NOT exist (dry run)
    assert!(!source.exists());
}

#[tokio::test]
async fn test_handle_undo_invalid_path() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Create undo log with some completed moves
    let undo_log_path = dir_path.join("undo_log.json");
    let mut undo_log = UndoLog::new();
    undo_log.record_move(
        PathBuf::from("/source/file.txt"),
        PathBuf::from("/dest/file.txt"),
    );
    undo_log.save(&undo_log_path).unwrap();

    // Use a non-existent path
    let invalid_path = dir_path.join("nonexistent_directory");
    let command = create_test_undo_command(false, Some(invalid_path.clone()));
    let result = handle_undo(&command, invalid_path).await;

    // Should handle error gracefully and return Ok
    assert!(result.is_ok());
}

// ============================================================================
// VALIDATE_AND_NORMALIZE_PATH TESTS
// ============================================================================

#[tokio::test]
async fn test_validate_and_normalize_path_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("nonexistent");

    let result = validate_and_normalize_path(&nonexistent).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("does not exist"));
}

#[tokio::test]
async fn test_validate_and_normalize_path_is_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    File::create(&file_path).unwrap();

    let result = validate_and_normalize_path(&file_path).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("not a directory"));
}

#[tokio::test]
async fn test_validate_and_normalize_path_success() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let result = validate_and_normalize_path(dir_path).await;

    assert!(result.is_ok());
    let normalized = result.unwrap();
    // The canonicalized path should be equivalent
    assert_eq!(normalized, dir_path.canonicalize().unwrap());
}

#[tokio::test]
async fn test_validate_and_normalize_path_empty_dir() {
    let temp_dir = TempDir::new().unwrap();
    let empty_dir = temp_dir.path();

    let result = validate_and_normalize_path(empty_dir).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_and_normalize_path_with_subdirs() {
    let temp_dir = TempDir::new().unwrap();
    let subdir = temp_dir.path().join("subdir").join("nested");
    fs::create_dir_all(&subdir).unwrap();

    let result = validate_and_normalize_path(&subdir).await;

    assert!(result.is_ok());
    let normalized = result.unwrap();
    assert!(normalized.to_string_lossy().contains("nested"));
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[tokio::test]
async fn test_handle_undo_multiple_moves_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Setup multiple files
    let images_dir = dir_path.join("Images");
    let docs_dir = dir_path.join("Documents");
    fs::create_dir_all(&images_dir).unwrap();
    fs::create_dir_all(&docs_dir).unwrap();

    let files = [
        (dir_path.join("photo1.jpg"), images_dir.join("photo1.jpg")),
        (dir_path.join("photo2.jpg"), images_dir.join("photo2.jpg")),
        (dir_path.join("doc1.pdf"), docs_dir.join("doc1.pdf")),
    ];

    for (source, dest) in &files {
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        File::create(dest).unwrap();
    }

    // Create undo log
    let undo_log_path = dir_path.join("undo_log.json");
    let moves: Vec<(PathBuf, PathBuf)> = files.iter().cloned().collect();
    create_undo_log_with_moves(&undo_log_path, moves);

    // Dry run
    let command = create_test_undo_command(true, None);
    let result = handle_undo(&command, dir_path).await;

    assert!(result.is_ok());
    // All destination files should still exist
    assert!(images_dir.join("photo1.jpg").exists());
    assert!(images_dir.join("photo2.jpg").exists());
    assert!(docs_dir.join("doc1.pdf").exists());
}

#[tokio::test]
async fn test_handle_undo_logs_saved() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    // Setup directory with file
    let images_dir = dir_path.join("Images");
    fs::create_dir_all(&images_dir).unwrap();
    let photo = images_dir.join("photo.jpg");
    let source = dir_path.join("photo.jpg");
    File::create(&photo).unwrap();

    // Create undo log
    let undo_log_path = dir_path.join("undo_log.json");
    create_undo_log_with_moves(&undo_log_path, vec![(source, photo.clone())]);

    // Create a new temp dir for target (to avoid cleanup issues)
    let target_temp = TempDir::new().unwrap();
    let target_path = target_temp.path().to_path_buf();

    // Copy the undo log to the new location
    fs::copy(&undo_log_path, target_path.join("undo_log.json")).unwrap();

    // Copy the file structure
    fs::create_dir_all(&target_path.join("Images")).unwrap();
    fs::copy(&photo, target_path.join("Images").join("photo.jpg")).unwrap();

    // Run undo with --dry-run to test it doesn't fail on save
    let command = create_test_undo_command(true, None);
    let result = handle_undo(&command, target_path).await;

    assert!(result.is_ok());
}
