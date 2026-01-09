use super::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
