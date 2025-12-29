use crate::files::*;
use std::fs::{self, File};
use std::io::Write;

#[test]
fn test_is_text_file_with_text_extensions() {
    assert!(is_text_file(Path::new("test.txt")));
    assert!(is_text_file(Path::new("test.rs")));
    assert!(is_text_file(Path::new("test.py")));
    assert!(is_text_file(Path::new("test.md")));
    assert!(is_text_file(Path::new("test.json")));
}

#[test]
fn test_is_text_file_with_binary_extensions() {
    assert!(!is_text_file(Path::new("test.exe")));
    assert!(!is_text_file(Path::new("test.bin")));
    assert!(!is_text_file(Path::new("test.jpg")));
    assert!(!is_text_file(Path::new("test.pdf")));
}

#[test]
fn test_is_text_file_case_insensitive() {
    assert!(is_text_file(Path::new("test.TXT")));
    assert!(is_text_file(Path::new("test.RS")));
    assert!(is_text_file(Path::new("test.Py")));
}

#[test]
fn test_file_batch_from_path() {
    let temp_dir = tempfile::tempdir().unwrap();
    let dir_path = temp_dir.path();

    File::create(dir_path.join("file1.txt")).unwrap();
    File::create(dir_path.join("file2.rs")).unwrap();
    fs::create_dir(dir_path.join("subdir")).unwrap();

    let batch = FileBatch::from_path(dir_path.to_path_buf(), false);
    assert_eq!(batch.count(), 2);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"file2.rs".to_string()));
}

#[test]
fn test_file_batch_from_path_nonexistent() {
    let batch = FileBatch::from_path(PathBuf::from("/nonexistent/path"), false);
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
    let batch = FileBatch::from_path(dir_path.to_path_buf(), false);
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
    let batch = FileBatch::from_path(dir_path.to_path_buf(), true);
    assert_eq!(batch.count(), 4);
    assert!(batch.filenames.contains(&"file1.txt".to_string()));
    assert!(batch.filenames.contains(&"subdir1/file2.rs".to_string()));
    assert!(
        batch
            .filenames
            .contains(&"subdir1/nested/file3.md".to_string())
    );
    assert!(batch.filenames.contains(&"subdir2/file4.py".to_string()));
}
#[test]
fn test_read_file_sample() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, World!").unwrap();

    let content = read_file_sample(&file_path, 1000);
    assert_eq!(content, Some("Hello, World!".to_string()));
}

#[test]
fn test_read_file_sample_with_limit() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(b"Hello, World! This is a long text.")
        .unwrap();

    let content = read_file_sample(&file_path, 5);
    assert_eq!(content, Some("Hello".to_string()));
}

#[test]
fn test_read_file_sample_binary_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.bin");

    let mut file = File::create(&file_path).unwrap();
    file.write_all(&[0x00, 0xFF, 0x80, 0x90]).unwrap();

    let content = read_file_sample(&file_path, 1000);
    assert_eq!(content, None);
}

#[test]
fn test_read_file_sample_nonexistent() {
    let content = read_file_sample(Path::new("/nonexistent/file.txt"), 1000);
    assert_eq!(content, None);
}

#[test]
fn test_organization_plan_serialization() {
    let plan = OrganizationPlan {
        files: vec![FileCategory {
            filename: "test.txt".to_string(),
            category: "Documents".to_string(),
            sub_category: "Text".to_string(),
        }],
    };

    let json = serde_json::to_string(&plan).unwrap();
    assert!(json.contains("test.txt"));
    assert!(json.contains("Documents"));

    let deserialized: OrganizationPlan = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.files[0].filename, "test.txt");
}

#[test]
fn test_file_category_serialization() {
    let fc = FileCategory {
        filename: "file.rs".to_string(),
        category: "Code".to_string(),
        sub_category: "Rust".to_string(),
    };

    let json = serde_json::to_string(&fc).unwrap();
    let deserialized: FileCategory = serde_json::from_str(&json).unwrap();

    assert_eq!(fc.filename, deserialized.filename);
    assert_eq!(fc.category, deserialized.category);
    assert_eq!(fc.sub_category, deserialized.sub_category);
}
