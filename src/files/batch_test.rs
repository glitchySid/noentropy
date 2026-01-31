use super::*;
use std::fs::{self, File};
use std::path::Path;

#[test]
fn test_file_batch_from_path() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path();

    File::create(dir_path.join("file1.txt")).unwrap();
    File::create(dir_path.join("file2.rs")).unwrap();
    fs::create_dir(dir_path.join("subdir")).unwrap();

    let batch = FileBatch::from_path(dir_path, false);
    assert_eq!(batch.count(), 2);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"file2.rs".to_string()));
}

#[test]
fn test_file_batch_from_path_nonexistent() {
    let batch = FileBatch::from_path(Path::new("/nonexistent/path"), false);
    assert_eq!(batch.count(), 0);
}

#[test]
fn test_file_batch_from_path_non_recursive() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path();
    File::create(dir_path.join("file1.txt")).unwrap();
    File::create(dir_path.join("file2.rs")).unwrap();
    fs::create_dir(dir_path.join("subdir")).unwrap();
    File::create(dir_path.join("subdir").join("file3.txt")).unwrap();
    let batch = FileBatch::from_path(dir_path, false);
    assert_eq!(batch.count(), 2);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"file2.rs".to_string()));
    assert!(!batch.filenames.contains(&"subdir/file3.txt".to_string()));
}

#[test]
fn test_file_batch_from_path_recursive() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path();
    File::create(dir_path.join("file1.txt")).unwrap();
    fs::create_dir(dir_path.join("subdir1")).unwrap();
    File::create(dir_path.join("subdir1").join("file2.rs")).unwrap();
    fs::create_dir(dir_path.join("subdir1").join("nested")).unwrap();
    File::create(dir_path.join("subdir1").join("nested").join("file3.md")).unwrap();
    fs::create_dir(dir_path.join("subdir2")).unwrap();
    File::create(dir_path.join("subdir2").join("file4.py")).unwrap();
    let batch = FileBatch::from_path(dir_path, true);
    assert_eq!(batch.count(), 4);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"subdir1/file2.rs".to_string()));
    assert!(batch
        .filenames
        .contains(&"subdir1/nested/file3.md".to_string()));
    assert!(batch.filenames.contains(&"subdir2/file4.py".to_string()));
}
