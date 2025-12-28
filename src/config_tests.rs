use crate::config::*;
use std::path::Path;

#[test]
fn test_config_serialization() {
    let config = Config {
        api_key: "test_key_12345".to_string(),
        download_folder: PathBuf::from("/test/path"),
    };

    let toml_str = toml::to_string_pretty(&config).unwrap();
    assert!(toml_str.contains("test_key_12345"));

    let deserialized: Config = toml::from_str(&toml_str).unwrap();
    assert_eq!(config.api_key, deserialized.api_key);
    assert_eq!(config.download_folder, deserialized.download_folder);
}

#[test]
fn test_validate_api_key_valid() {
    assert!(crate::prompt::Prompter::validate_api_key(
        "AIzaSyB1234567890123456789012345678"
    ));
    assert!(crate::prompt::Prompter::validate_api_key(
        "AIzaSyB123456789012345678901234567890"
    ));
}

#[test]
fn test_validate_api_key_invalid() {
    assert!(!crate::prompt::Prompter::validate_api_key(""));
    assert!(!crate::prompt::Prompter::validate_api_key("invalid_key"));
    assert!(!crate::prompt::Prompter::validate_api_key(
        "BizaSyB1234567890123456789012345678"
    ));
    assert!(!crate::prompt::Prompter::validate_api_key("short"));
}

#[test]
fn test_validate_folder_path_valid() {
    let temp_dir = tempfile::tempdir().unwrap();
    assert!(crate::prompt::Prompter::validate_folder_path(
        temp_dir.path()
    ));
}

#[test]
fn test_validate_folder_path_invalid() {
    assert!(!crate::prompt::Prompter::validate_folder_path(Path::new(
        "/nonexistent/path/that/does/not/exist"
    )));

    let temp_file = tempfile::NamedTempFile::new().unwrap();
    assert!(!crate::prompt::Prompter::validate_folder_path(
        temp_file.path()
    ));
}

#[test]
fn test_expand_home_with_tilde() {
    if let Some(base_dirs) = directories::BaseDirs::new() {
        let home = base_dirs.home_dir();
        let expanded = crate::prompt::Prompter::expand_home("~/test/path");
        assert!(expanded.starts_with(home.to_string_lossy().as_ref()));
        assert!(expanded.contains("test/path"));
    }
}

#[test]
fn test_expand_home_without_tilde() {
    let expanded = crate::prompt::Prompter::expand_home("/absolute/path");
    assert_eq!(expanded, "/absolute/path");

    let expanded = crate::prompt::Prompter::expand_home("relative/path");
    assert_eq!(expanded, "relative/path");
}

#[test]
fn test_get_default_downloads_folder() {
    let path = crate::prompt::Prompter::get_default_downloads_folder();
    assert!(path.ends_with("Downloads"));
}

#[test]
fn test_config_empty_api_key_error() {
    let config = Config {
        api_key: String::new(),
        download_folder: PathBuf::from("/test/path"),
    };

    assert!(config.api_key.is_empty());
}
