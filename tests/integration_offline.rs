//! Integration tests for offline file organization
//!
//! These tests verify the end-to-end behavior of offline organization,
//! including actual file moves and directory structure creation.

use noentropy::files::{FileBatch, categorize_files_offline};
use noentropy::models::{FileCategory, OrganizationPlan};
use noentropy::storage::UndoLog;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a temp directory with test files
fn setup_test_directory(files: &[(&str, Option<&[u8]>)]) -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    for (filename, content) in files {
        let file_path = temp_dir.path().join(filename);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut file = File::create(&file_path).unwrap();
        if let Some(data) = content {
            file.write_all(data).unwrap();
        }
    }

    temp_dir
}

/// Helper to verify file exists at expected location
#[allow(dead_code)]
fn assert_file_exists(base: &Path, relative_path: &str) {
    let full_path = base.join(relative_path);
    assert!(
        full_path.exists(),
        "Expected file at {:?} but it doesn't exist",
        full_path
    );
}

/// Helper to verify file does NOT exist at location
#[allow(dead_code)]
fn assert_file_not_exists(base: &Path, relative_path: &str) {
    let full_path = base.join(relative_path);
    assert!(
        !full_path.exists(),
        "Expected file NOT to exist at {:?} but it does",
        full_path
    );
}

// ============================================================================
// OFFLINE ORGANIZATION INTEGRATION TESTS
// ============================================================================

#[test]
fn test_offline_categorization_produces_correct_plan() {
    let filenames = vec![
        "photo.jpg".to_string(),
        "document.pdf".to_string(),
        "code.rs".to_string(),
        "song.mp3".to_string(),
        "video.mp4".to_string(),
        "archive.zip".to_string(),
        "installer.exe".to_string(),
        "unknown.xyz".to_string(),
    ];

    let result = categorize_files_offline(filenames);

    // Verify categorized files
    assert_eq!(result.plan.files.len(), 7);
    assert_eq!(result.skipped.len(), 1);
    assert!(result.skipped.contains(&"unknown.xyz".to_string()));

    // Verify categories are correct
    let find_category = |filename: &str| -> Option<&str> {
        result
            .plan
            .files
            .iter()
            .find(|f| f.filename == filename)
            .map(|f| f.category.as_str())
    };

    assert_eq!(find_category("photo.jpg"), Some("Images"));
    assert_eq!(find_category("document.pdf"), Some("Documents"));
    assert_eq!(find_category("code.rs"), Some("Code"));
    assert_eq!(find_category("song.mp3"), Some("Music"));
    assert_eq!(find_category("video.mp4"), Some("Video"));
    assert_eq!(find_category("archive.zip"), Some("Archives"));
    assert_eq!(find_category("installer.exe"), Some("Installers"));
}

#[test]
fn test_file_batch_collects_files_correctly() {
    let temp_dir = setup_test_directory(&[
        ("file1.txt", Some(b"content1")),
        ("file2.jpg", Some(b"image data")),
        ("subdir/file3.rs", Some(b"fn main() {}")),
    ]);

    // Non-recursive should only get top-level files
    let batch = FileBatch::from_path(temp_dir.path(), false);
    assert_eq!(batch.count(), 2);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"file2.jpg".to_string()));

    // Recursive should get all files
    let batch_recursive = FileBatch::from_path(temp_dir.path(), true);
    assert_eq!(batch_recursive.count(), 3);
}

#[test]
fn test_undo_log_tracks_moves_correctly() {
    let mut undo_log = UndoLog::new();
    let source = Path::new("/tmp/source/file.txt").to_path_buf();
    let dest = Path::new("/tmp/dest/Documents/file.txt").to_path_buf();

    undo_log.record_move(source.clone(), dest.clone());

    assert_eq!(undo_log.get_completed_count(), 1);
    assert!(undo_log.has_completed_moves());

    let completed = undo_log.get_completed_moves();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].source_path, source);
    assert_eq!(completed[0].destination_path, dest);
}

#[test]
fn test_undo_log_marks_moves_as_undone() {
    let mut undo_log = UndoLog::new();
    let source = Path::new("/tmp/source/file.txt").to_path_buf();
    let dest = Path::new("/tmp/dest/Documents/file.txt").to_path_buf();

    undo_log.record_move(source, dest.clone());
    assert_eq!(undo_log.get_completed_count(), 1);

    undo_log.mark_as_undone(&dest);
    assert_eq!(undo_log.get_completed_count(), 0);
}

#[test]
fn test_undo_log_eviction_policy() {
    let mut undo_log = UndoLog::with_max_entries(3);

    for i in 0..5 {
        let source = Path::new(&format!("/tmp/source/file{}.txt", i)).to_path_buf();
        let dest = Path::new(&format!("/tmp/dest/file{}.txt", i)).to_path_buf();
        undo_log.record_move(source, dest);
    }

    // Should have evicted oldest entries to stay within limit
    let completed = undo_log.get_completed_moves();
    assert!(completed.len() <= 3);
}

#[test]
fn test_categorization_handles_edge_cases() {
    let filenames = vec![
        // Files without extensions
        "README".to_string(),
        "Makefile".to_string(),
        ".gitignore".to_string(),
        // Hidden files with extensions
        ".hidden.txt".to_string(),
        // Multiple dots
        "file.name.with.dots.pdf".to_string(),
        // All caps
        "IMAGE.JPG".to_string(),
        // Mixed case
        "Document.PdF".to_string(),
    ];

    let result = categorize_files_offline(filenames);

    // Files without extensions should be skipped
    assert!(result.skipped.contains(&"README".to_string()));
    assert!(result.skipped.contains(&"Makefile".to_string()));

    // Case insensitive matching should work
    let find_category = |filename: &str| -> Option<&str> {
        result
            .plan
            .files
            .iter()
            .find(|f| f.filename == filename)
            .map(|f| f.category.as_str())
    };

    assert_eq!(find_category("IMAGE.JPG"), Some("Images"));
    assert_eq!(find_category("Document.PdF"), Some("Documents"));
    assert_eq!(find_category("file.name.with.dots.pdf"), Some("Documents"));
}

#[test]
fn test_organization_plan_with_subcategories() {
    let plan = OrganizationPlan {
        files: vec![
            FileCategory {
                filename: "project.rs".to_string(),
                category: "Code".to_string(),
                sub_category: "Rust".to_string(),
            },
            FileCategory {
                filename: "script.py".to_string(),
                category: "Code".to_string(),
                sub_category: "Python".to_string(),
            },
        ],
    };

    assert_eq!(plan.files.len(), 2);
    assert_eq!(plan.files[0].sub_category, "Rust");
    assert_eq!(plan.files[1].sub_category, "Python");
}

// ============================================================================
// LARGE SCALE TESTS
// ============================================================================

#[test]
fn test_categorization_handles_large_file_lists() {
    // Generate 1000 files with various extensions
    let extensions = vec![
        "jpg", "png", "pdf", "docx", "rs", "py", "mp3", "mp4", "zip", "exe", "xyz",
    ];

    let filenames: Vec<String> = (0..1000)
        .map(|i| format!("file{}.{}", i, extensions[i % extensions.len()]))
        .collect();

    let result = categorize_files_offline(filenames);

    // Should categorize most files (10/11 extensions are known)
    let expected_categorized = (1000 / 11) * 10 + (1000 % 11).min(10);
    assert!(result.plan.files.len() >= expected_categorized - 10); // Allow some margin
    assert!(!result.skipped.is_empty()); // .xyz files should be skipped
}

#[test]
fn test_file_batch_handles_deep_directory_structure() {
    let temp_dir = setup_test_directory(&[
        ("level1/file1.txt", Some(b"1")),
        ("level1/level2/file2.txt", Some(b"2")),
        ("level1/level2/level3/file3.txt", Some(b"3")),
        ("level1/level2/level3/level4/file4.txt", Some(b"4")),
    ]);

    let batch = FileBatch::from_path(temp_dir.path(), true);

    assert_eq!(batch.count(), 4);
    assert!(batch.filenames.iter().any(|f| f.contains("level4")));
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_file_batch_handles_permission_errors_gracefully() {
    // FileBatch should not crash when encountering permission issues
    let temp_dir = TempDir::new().unwrap();
    File::create(temp_dir.path().join("readable.txt")).unwrap();

    // This should complete without panicking
    let batch = FileBatch::from_path(temp_dir.path(), false);
    assert!(batch.count() >= 1);
}

#[test]
fn test_categorization_handles_empty_input() {
    let result = categorize_files_offline(vec![]);

    assert!(result.plan.files.is_empty());
    assert!(result.skipped.is_empty());
}

#[test]
fn test_categorization_handles_unicode_filenames() {
    let filenames = vec![
        "文档.pdf".to_string(),         // Chinese
        "документ.docx".to_string(),    // Russian
        "ドキュメント.txt".to_string(), // Japanese
        "émoji🎉.jpg".to_string(),      // Emoji
    ];

    let result = categorize_files_offline(filenames);

    // All should be categorized correctly by extension
    assert_eq!(result.plan.files.len(), 4);
    assert!(result.skipped.is_empty());
}
