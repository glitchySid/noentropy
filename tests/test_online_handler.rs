//! Unit tests for handle_online_organization handler
//!
//! Tests the online (AI-powered) file organization functionality including:
//! - Args and Config creation
//! - FileBatch handling
//! - Text file detection for deep inspection
//! - File sample reading
//! - API error handling (graceful degradation)

use noentropy::cli::args::Command;
use noentropy::cli::handlers::handle_online_organization;
use noentropy::files::{FileBatch, is_text_file, read_file_sample};
use noentropy::settings::Config;
use noentropy::storage::{Cache, UndoLog};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Helper to create test Command::Organize
fn create_test_organize_command(dry_run: bool, max_concurrent: usize) -> Command {
    Command::Organize {
        dry_run,
        max_concurrent,
        offline: false,
        recursive: false,
        path: None,
    }
}

/// Helper to create a test Config
fn create_test_config(api_key: &str) -> Config {
    Config {
        api_key: api_key.to_string(),
        download_folder: PathBuf::new(),
        categories: vec![
            "Images".to_string(),
            "Documents".to_string(),
            "Code".to_string(),
            "Music".to_string(),
            "Video".to_string(),
            "Archives".to_string(),
        ],
    }
}

/// Helper to create a FileBatch from filenames
fn create_file_batch(filenames: Vec<String>, base_path: &Path) -> FileBatch {
    let paths: Vec<PathBuf> = filenames.iter().map(|f| base_path.join(f)).collect();
    FileBatch { filenames, paths }
}

/// Helper to setup a temp directory with test files
fn setup_test_dir_with_files(files: &[(&str, Option<&str>)]) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    for (filename, content) in files {
        let file_path = dir_path.join(filename);
        let mut file = File::create(&file_path).unwrap();
        if let Some(text) = content {
            file.write_all(text.as_bytes()).unwrap();
        }
    }

    (temp_dir, dir_path)
}

// ============================================================================
// ARGS/COMMAND TESTS
// ============================================================================

#[test]
fn test_command_organize_creation() {
    let command = create_test_organize_command(true, 10);
    match &command {
        Command::Organize {
            dry_run,
            max_concurrent,
            offline,
            recursive,
            path: _,
        } => {
            assert!(*dry_run);
            assert_eq!(*max_concurrent, 10);
            assert!(!*offline);
            assert!(!*recursive);
        }
        _ => panic!("Expected Command::Organize"),
    }
}

#[test]
fn test_command_organize_default_max_concurrent() {
    let command = create_test_organize_command(false, 5);
    match &command {
        Command::Organize { max_concurrent, .. } => {
            assert_eq!(*max_concurrent, 5);
        }
        _ => panic!("Expected Command::Organize"),
    }
}

#[test]
fn test_command_organize_all_flags() {
    let command = Command::Organize {
        dry_run: true,
        max_concurrent: 10,
        recursive: true,
        offline: true,
        path: Some(PathBuf::from("/test/path")),
    };

    match &command {
        Command::Organize {
            dry_run,
            max_concurrent: _,
            recursive,
            offline: _,
            path,
        } => {
            assert!(*dry_run);
            assert!(*recursive);
            assert_eq!(path, &Some(PathBuf::from("/test/path")));
        }
        _ => panic!("Expected Command::Organize"),
    }
}

// ============================================================================
// CONFIG TESTS
// ============================================================================

#[test]
fn test_config_creation() {
    let config = create_test_config("test-api-key");
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.categories.len(), 6);
    assert!(config.categories.contains(&"Images".to_string()));
}

#[test]
fn test_config_with_custom_categories() {
    let config = Config {
        api_key: "key".to_string(),
        download_folder: PathBuf::from("/test"),
        categories: vec!["Custom1".to_string(), "Custom2".to_string()],
    };

    assert_eq!(config.categories.len(), 2);
    assert!(config.categories.contains(&"Custom1".to_string()));
}

#[test]
fn test_config_empty_categories() {
    let config = Config {
        api_key: "key".to_string(),
        download_folder: PathBuf::new(),
        categories: vec![],
    };

    assert!(config.categories.is_empty());
}

// ============================================================================
// FILE BATCH TESTS
// ============================================================================

#[test]
fn test_file_batch_creation() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();

    let filenames = vec!["test.txt".to_string(), "image.jpg".to_string()];
    let batch = create_file_batch(filenames.clone(), dir_path);

    assert_eq!(batch.filenames.len(), 2);
    assert_eq!(batch.paths.len(), 2);
    assert!(batch.paths[0].ends_with("test.txt"));
    assert!(batch.paths[1].ends_with("image.jpg"));
}

#[test]
fn test_file_batch_empty() {
    let temp_dir = TempDir::new().unwrap();
    let batch = create_file_batch(vec![], temp_dir.path());

    assert!(batch.filenames.is_empty());
    assert!(batch.paths.is_empty());
}

#[test]
fn test_file_batch_count() {
    let temp_dir = TempDir::new().unwrap();
    let filenames: Vec<String> = (0..10).map(|i| format!("file{}.txt", i)).collect();
    let batch = create_file_batch(filenames, temp_dir.path());

    assert_eq!(batch.count(), 10);
}

// ============================================================================
// TEXT FILE DETECTION TESTS (for deep inspection)
// ============================================================================

#[test]
fn test_text_file_detection_for_deep_inspection() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[
        ("test.txt", Some("text content")),
        ("test.rs", Some("fn main() {}")),
        ("test.jpg", None),
    ]);

    // Text files should be detected for deep inspection
    assert!(is_text_file(&dir_path.join("test.txt")));
    assert!(is_text_file(&dir_path.join("test.rs")));
}

#[test]
fn test_text_file_detection_various_extensions() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[
        ("code.py", Some("print('hello')")),
        ("code.js", Some("console.log('hi')")),
        ("config.json", Some("{}")),
        ("config.yaml", Some("key: value")),
        ("doc.md", Some("# Title")),
    ]);

    assert!(is_text_file(&dir_path.join("code.py")));
    assert!(is_text_file(&dir_path.join("code.js")));
    assert!(is_text_file(&dir_path.join("config.json")));
    assert!(is_text_file(&dir_path.join("config.yaml")));
    assert!(is_text_file(&dir_path.join("doc.md")));
}

// ============================================================================
// FILE SAMPLE READING TESTS
// ============================================================================

#[test]
fn test_read_file_sample_for_deep_inspection() {
    let (_temp_dir, dir_path) =
        setup_test_dir_with_files(&[("test.txt", Some("This is a test file with some content."))]);

    let sample = read_file_sample(&dir_path.join("test.txt"), 100);
    assert!(sample.is_some());
    assert!(sample.unwrap().contains("test file"));
}

#[test]
fn test_read_file_sample_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let sample = read_file_sample(&temp_dir.path().join("nonexistent.txt"), 100);
    assert!(sample.is_none());
}

#[test]
fn test_read_file_sample_truncation() {
    let long_content = "x".repeat(1000);
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[("long.txt", Some(&long_content))]);

    let sample = read_file_sample(&dir_path.join("long.txt"), 100);
    assert!(sample.is_some());
    let sample_content = sample.unwrap();
    assert!(sample_content.len() <= 100);
}

#[test]
fn test_read_file_sample_empty_file() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[("empty.txt", Some(""))]);

    let sample = read_file_sample(&dir_path.join("empty.txt"), 100);
    assert!(sample.is_some());
    assert!(sample.unwrap().is_empty());
}

#[test]
fn test_read_file_sample_exact_limit() {
    let content = "x".repeat(100);
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[("exact.txt", Some(&content))]);

    let sample = read_file_sample(&dir_path.join("exact.txt"), 100);
    assert!(sample.is_some());
    assert_eq!(sample.unwrap().len(), 100);
}

// ============================================================================
// HANDLER ASYNC TESTS
// ============================================================================

#[tokio::test]
async fn test_handle_online_organization_requires_valid_api_key() {
    // This test validates that the function correctly handles API setup
    // In a real scenario, an invalid API key would result in an API error
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[("test.txt", Some("content"))]);
    let command = create_test_organize_command(true, 5);
    let config = create_test_config("invalid-api-key");
    let batch = create_file_batch(vec!["test.txt".to_string()], &dir_path);
    let mut cache = Cache::new();
    let mut undo_log = UndoLog::new();

    // The function should attempt to call the API
    // With an invalid key, it will fail but should handle the error gracefully
    let result = handle_online_organization(
        &command,
        &config,
        batch,
        &dir_path,
        &mut cache,
        &mut undo_log,
    )
    .await;

    // The function returns Ok(None) even on API errors (handled internally)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_online_organization_empty_batch() {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path();
    let command = create_test_organize_command(true, 5);
    let config = create_test_config("test-key");
    let batch = create_file_batch(vec![], dir_path);
    let mut cache = Cache::new();
    let mut undo_log = UndoLog::new();

    // Empty batch should be handled gracefully
    let result = handle_online_organization(
        &command,
        &config,
        batch,
        dir_path,
        &mut cache,
        &mut undo_log,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_online_organization_dry_run() {
    let (_temp_dir, dir_path) =
        setup_test_dir_with_files(&[("photo.jpg", Some("image")), ("document.pdf", Some("pdf"))]);
    let command = create_test_organize_command(true, 5); // dry_run = true
    let config = create_test_config("test-key");
    let batch = create_file_batch(
        vec!["photo.jpg".to_string(), "document.pdf".to_string()],
        &dir_path,
    );
    let mut cache = Cache::new();
    let mut undo_log = UndoLog::new();

    let result = handle_online_organization(
        &command,
        &config,
        batch,
        &dir_path,
        &mut cache,
        &mut undo_log,
    )
    .await;

    assert!(result.is_ok());
    // Files should still exist (dry run + API failure = no moves)
    assert!(dir_path.join("photo.jpg").exists());
    assert!(dir_path.join("document.pdf").exists());
}

// ============================================================================
// CACHE AND UNDO LOG TESTS
// ============================================================================

#[test]
fn test_cache_new() {
    let cache = Cache::new();
    // Just verify it can be created
    assert!(true);
    let _ = cache; // Use the variable to avoid warning
}

#[test]
fn test_undo_log_new() {
    let undo_log = UndoLog::new();
    assert_eq!(undo_log.get_completed_count(), 0);
    assert!(!undo_log.has_completed_moves());
}

#[test]
fn test_undo_log_record_move() {
    let mut undo_log = UndoLog::new();
    undo_log.record_move(
        PathBuf::from("/source/file.txt"),
        PathBuf::from("/dest/file.txt"),
    );

    assert_eq!(undo_log.get_completed_count(), 1);
    assert!(undo_log.has_completed_moves());
}
