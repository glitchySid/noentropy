//! Unit tests for handle_offline_organization handler
//!
//! Tests the offline file organization functionality including:
//! - Empty batch handling
//! - Unknown extension handling
//! - Dry run behavior
//! - Various file extension categorization
//! - Undo log behavior
//! - Helper function behavior

use noentropy::cli::handlers::handle_offline_organization;
use noentropy::files::FileBatch;
use noentropy::models::{FileCategory, OrganizationPlan};
use noentropy::storage::UndoLog;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Helper to create a temporary directory with test files
fn setup_test_dir_with_files(files: &[&str]) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let dir_path = temp_dir.path().to_path_buf();

    for filename in files {
        let file_path = dir_path.join(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        File::create(&file_path).unwrap();
    }

    (temp_dir, dir_path)
}

/// Helper to create a FileBatch from a list of filenames and a base path
fn create_file_batch(filenames: Vec<String>, base_path: &Path) -> FileBatch {
    let paths: Vec<PathBuf> = filenames.iter().map(|f| base_path.join(f)).collect();
    FileBatch { filenames, paths }
}

// ============================================================================
// HANDLER TESTS
// ============================================================================

#[test]
fn test_handle_offline_organization_empty_batch() {
    let temp_dir = TempDir::new().unwrap();
    let target_path = temp_dir.path();
    let mut undo_log = UndoLog::new();

    let batch = FileBatch {
        filenames: vec![],
        paths: vec![],
    };

    let result = handle_offline_organization(batch, target_path, true, &mut undo_log);

    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_handle_offline_organization_all_unknown_extensions() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["file1.xyz", "file2.unknown"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec!["file1.xyz".to_string(), "file2.unknown".to_string()],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
    // Should return None when no files can be categorized
    assert!(result.unwrap().is_none());
}

#[test]
fn test_handle_offline_organization_dry_run_no_file_moves() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["photo.jpg", "document.pdf"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec!["photo.jpg".to_string(), "document.pdf".to_string()],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
    // In dry run, files should NOT be moved
    assert!(dir_path.join("photo.jpg").exists());
    assert!(dir_path.join("document.pdf").exists());
    // Destination folders should NOT be created
    assert!(!dir_path.join("Images").exists());
    assert!(!dir_path.join("Documents").exists());
}

#[test]
fn test_handle_offline_organization_mixed_files() {
    let (_temp_dir, dir_path) =
        setup_test_dir_with_files(&["photo.jpg", "document.pdf", "unknown.xyz", "song.mp3"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec![
            "photo.jpg".to_string(),
            "document.pdf".to_string(),
            "unknown.xyz".to_string(),
            "song.mp3".to_string(),
        ],
        &dir_path,
    );

    // Dry run to verify categorization without moving
    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
    // Files should still exist (dry run)
    assert!(dir_path.join("photo.jpg").exists());
    assert!(dir_path.join("document.pdf").exists());
    assert!(dir_path.join("unknown.xyz").exists());
    assert!(dir_path.join("song.mp3").exists());
}

#[test]
fn test_handle_offline_organization_various_extensions() {
    let files = vec![
        // Images
        "test.png",
        "test.gif",
        "test.webp",
        // Documents
        "test.docx",
        "test.xlsx",
        "test.txt",
        // Code
        "test.rs",
        "test.py",
        "test.js",
        // Archives
        "test.zip",
        "test.tar",
        // Video
        "test.mp4",
        "test.mkv",
        // Music
        "test.wav",
        "test.flac",
        // Installers
        "test.exe",
        "test.dmg",
    ];

    let (_temp_dir, dir_path) = setup_test_dir_with_files(&files);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(files.iter().map(|s| s.to_string()).collect(), &dir_path);

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}

#[test]
fn test_handle_offline_organization_case_insensitive() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["PHOTO.JPG", "Document.PDF"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec!["PHOTO.JPG".to_string(), "Document.PDF".to_string()],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}

#[test]
fn test_handle_offline_organization_undo_log_not_modified_in_dry_run() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["photo.jpg"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(vec!["photo.jpg".to_string()], &dir_path);

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
    // Undo log should be empty in dry run mode
    assert_eq!(undo_log.get_completed_count(), 0);
}

#[test]
fn test_handle_offline_organization_files_without_extension() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["README", "Makefile", ".gitignore"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec![
            "README".to_string(),
            "Makefile".to_string(),
            ".gitignore".to_string(),
        ],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
    // All files have no/unknown extensions, should return None
    assert!(result.unwrap().is_none());
}

// ============================================================================
// ORGANIZATION PLAN TESTS
// ============================================================================

#[test]
fn test_organization_plan_structure() {
    let plan = OrganizationPlan {
        files: vec![
            FileCategory {
                filename: "photo1.jpg".to_string(),
                category: "Images".to_string(),
                sub_category: String::new(),
            },
            FileCategory {
                filename: "photo2.png".to_string(),
                category: "Images".to_string(),
                sub_category: String::new(),
            },
            FileCategory {
                filename: "doc.pdf".to_string(),
                category: "Documents".to_string(),
                sub_category: String::new(),
            },
        ],
    };

    assert_eq!(plan.files.len(), 3);
    assert_eq!(plan.files[0].category, "Images");
    assert_eq!(plan.files[2].category, "Documents");
}

#[test]
fn test_organization_plan_empty() {
    let plan = OrganizationPlan { files: vec![] };

    assert!(plan.files.is_empty());
}

#[test]
fn test_file_category_with_subcategory() {
    let file_category = FileCategory {
        filename: "project.rs".to_string(),
        category: "Code".to_string(),
        sub_category: "Rust".to_string(),
    };

    assert_eq!(file_category.filename, "project.rs");
    assert_eq!(file_category.category, "Code");
    assert_eq!(file_category.sub_category, "Rust");
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_handle_offline_organization_hidden_files() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&[".hidden.txt", ".config.json"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec![".hidden.txt".to_string(), ".config.json".to_string()],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}

#[test]
fn test_handle_offline_organization_multiple_dots_in_filename() {
    let (_temp_dir, dir_path) =
        setup_test_dir_with_files(&["file.name.with.dots.pdf", "archive.tar.gz"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(
        vec![
            "file.name.with.dots.pdf".to_string(),
            "archive.tar.gz".to_string(),
        ],
        &dir_path,
    );

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}

#[test]
fn test_handle_offline_organization_single_file() {
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&["single.jpg"]);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(vec!["single.jpg".to_string()], &dir_path);

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}

#[test]
fn test_handle_offline_organization_large_batch() {
    // Generate 100 files with various extensions
    let extensions = vec!["jpg", "pdf", "rs", "mp3", "mp4", "zip"];
    let files: Vec<String> = (0..100)
        .map(|i| format!("file{}.{}", i, extensions[i % extensions.len()]))
        .collect();

    let file_refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();
    let (_temp_dir, dir_path) = setup_test_dir_with_files(&file_refs);
    let mut undo_log = UndoLog::new();

    let batch = create_file_batch(files, &dir_path);

    let result = handle_offline_organization(batch, &dir_path, true, &mut undo_log);

    assert!(result.is_ok());
}
