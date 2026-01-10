use super::*;

#[test]
fn test_categorize_known_extensions() {
    assert_eq!(categorize_by_extension("photo.jpg"), Some("Images"));
    assert_eq!(categorize_by_extension("document.pdf"), Some("Documents"));
    assert_eq!(categorize_by_extension("setup.exe"), Some("Installers"));
    assert_eq!(categorize_by_extension("song.mp3"), Some("Music"));
    assert_eq!(categorize_by_extension("movie.mp4"), Some("Video"));
    assert_eq!(categorize_by_extension("archive.zip"), Some("Archives"));
    assert_eq!(categorize_by_extension("main.rs"), Some("Code"));
}

#[test]
fn test_categorize_case_insensitive() {
    assert_eq!(categorize_by_extension("PHOTO.JPG"), Some("Images"));
    assert_eq!(categorize_by_extension("Photo.Png"), Some("Images"));
}

#[test]
fn test_categorize_unknown_extension() {
    assert_eq!(categorize_by_extension("file.xyz"), None);
    assert_eq!(categorize_by_extension("file.unknown"), None);
}

#[test]
fn test_categorize_no_extension() {
    assert_eq!(categorize_by_extension("README"), None);
    assert_eq!(categorize_by_extension("Makefile"), None);
}

#[test]
fn test_categorize_files_offline() {
    let filenames = vec![
        "photo.jpg".to_string(),
        "doc.pdf".to_string(),
        "unknown".to_string(),
        "file.xyz".to_string(),
    ];

    let result = categorize_files_offline(filenames);

    assert_eq!(result.plan.files.len(), 2);
    assert_eq!(result.skipped.len(), 2);
    assert!(result.skipped.contains(&"unknown".to_string()));
    assert!(result.skipped.contains(&"file.xyz".to_string()));
}
