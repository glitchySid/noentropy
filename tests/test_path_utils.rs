//! Unit tests for path_utils module
//!
//! Tests the validate_and_normalize_path function including:
//! - Non-existent paths
//! - File paths (not directories)
//! - Directory validation
//! - Successful canonicalization
//! - Permission/access errors

use noentropy::cli::path_utils::validate_and_normalize_path;
use std::fs::{self, File};
use std::path::Path;
use tempfile::TempDir;

// ============================================================================
// NON-EXISTENT PATH TESTS
// ============================================================================

#[tokio::test]
async fn test_validate_nonexistent_path() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("this_does_not_exist");

    let result = validate_and_normalize_path(&nonexistent).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("does not exist"));
}

#[tokio::test]
async fn test_validate_deeply_nested_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir
        .path()
        .join("a")
        .join("b")
        .join("c")
        .join("d")
        .join("nonexistent");

    let result = validate_and_normalize_path(&nonexistent).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

// ============================================================================
// FILE PATH TESTS (NOT DIRECTORY)
// ============================================================================

#[tokio::test]
async fn test_validate_file_path_is_not_directory() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    File::create(&file_path).unwrap();

    let result = validate_and_normalize_path(&file_path).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("not a directory"));
}

#[tokio::test]
async fn test_validate_file_path_with_nested_structure() {
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("dir1").join("dir2");
    fs::create_dir_all(&nested_dir).unwrap();

    let file_in_nested = nested_dir.join("file.txt");
    File::create(&file_in_nested).unwrap();

    let result = validate_and_normalize_path(&file_in_nested).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not a directory"));
}

// ============================================================================
// DIRECTORY VALIDATION TESTS
// ============================================================================

#[tokio::test]
async fn test_validate_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let empty_dir = temp_dir.path();

    let result = validate_and_normalize_path(empty_dir).await;

    assert!(result.is_ok());
    let normalized = result.unwrap();
    // Should be canonicalized to an absolute path
    assert!(normalized.is_absolute());
}

#[tokio::test]
async fn test_validate_directory_with_files() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create some files
    File::create(dir_path.join("file1.txt")).unwrap();
    File::create(dir_path.join("file2.txt")).unwrap();

    let result = validate_and_normalize_path(dir_path).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_directory_with_subdirectories() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create nested directory structure
    fs::create_dir_all(dir_path.join("subdir1").join("subdir2")).unwrap();
    File::create(dir_path.join("file.txt")).unwrap();

    let result = validate_and_normalize_path(dir_path).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_directory_with_hidden_files() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create hidden files
    File::create(dir_path.join(".gitignore")).unwrap();
    File::create(dir_path.join(".config")).unwrap();

    let result = validate_and_normalize_path(dir_path).await;

    assert!(result.is_ok());
}

// ============================================================================
// PATH NORMALIZATION TESTS
// ============================================================================

#[tokio::test]
async fn test_validate_normalizes_relative_path() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Get canonical path first
    let canonical = dir_path.canonicalize().unwrap();

    let result = validate_and_normalize_path(dir_path).await;

    assert!(result.is_ok());
    let normalized = result.unwrap();
    // The normalized path should be equivalent to canonicalized path
    assert_eq!(normalized, canonical);
}

#[tokio::test]
async fn test_validate_resolves_dot_path() {
    let temp_dir = TempDir::new().unwrap();
    let original_cwd = std::env::current_dir().unwrap();

    // Change to temp directory
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Test with "./" path
    let dot_path = Path::new(".");
    let result = validate_and_normalize_path(dot_path).await;

    // Restore original directory
    std::env::set_current_dir(&original_cwd).unwrap();

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_directory_symlink_if_available() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create a real directory
    let real_dir = dir_path.join("real_dir");
    fs::create_dir_all(&real_dir).unwrap();

    // Create a symlink to it (may not work on all platforms)
    #[cfg(unix)]
    {
        let symlink_path = dir_path.join("symlink_dir");
        if let Ok(()) = std::os::unix::fs::symlink(&real_dir, &symlink_path) {
            let result = validate_and_normalize_path(&symlink_path).await;
            // Should resolve to the canonical path of the real directory
            assert!(result.is_ok());
        }
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[tokio::test]
async fn test_validate_root_directory() {
    // Test with root directory (may not be readable on all systems)
    let root_path = Path::new("/");

    let result = validate_and_normalize_path(root_path).await;

    // On most systems, root should be accessible
    // If it fails, it's likely due to permissions, not path validation
    match result {
        Ok(_) => {
            // Root is accessible and canonicalizable
        }
        Err(e) => {
            // Should be an access error, not "does not exist" or "not a directory"
            assert!(!e.contains("does not exist"));
            assert!(!e.contains("not a directory"));
        }
    }
}

#[tokio::test]
async fn test_validate_current_directory() {
    let current_dir = Path::new(".");

    let result = validate_and_normalize_path(current_dir).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_parent_directory() {
    let parent_dir = Path::new("..");

    let result = validate_and_normalize_path(parent_dir).await;

    // Parent directory should be valid (assuming we have read access)
    match result {
        Ok(normalized) => {
            // Should be an absolute path
            assert!(normalized.is_absolute());
        }
        Err(e) => {
            // If it fails, it should be an access error
            assert!(e.contains("Cannot access") || e.contains("access"));
        }
    }
}

#[tokio::test]
async fn test_validate_directory_with_special_characters() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create directory with special characters in name
    let special_dir = dir_path.join("dir-with-dashes_and_underscores");
    fs::create_dir_all(&special_dir).unwrap();

    let result = validate_and_normalize_path(&special_dir).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_unicode_directory_name() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    // Create directory with unicode characters
    let unicode_dir = dir_path.join("测试目录");
    fs::create_dir_all(&unicode_dir).unwrap();

    let result = validate_and_normalize_path(&unicode_dir).await;

    assert!(result.is_ok());
}
