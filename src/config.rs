use colored::*;
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const MAX_RETRIES: u32 = 3;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    pub download_folder: PathBuf,
}

impl Config {
    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("dev", "noentropy", "NoEntropy") {
            let config_dir = proj_dirs.config_dir().to_path_buf();
            fs::create_dir_all(&config_dir)?;
            Ok(config_dir)
        } else {
            Err("Failed to determine config directory".into())
        }
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_config_dir()?.join("config.toml"))
    }

    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err("Config file not found".into());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;

        if config.api_key.is_empty() {
            return Err("API key not found in config file".into());
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        let toml_string = toml::to_string_pretty(self)?;

        fs::write(&config_path, toml_string)?;

        println!(
            "{} Configuration saved to {}",
            "✓".green(),
            config_path.display().to_string().yellow()
        );

        Ok(())
    }

    pub fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
        match Self::load() {
            Ok(config) => Ok(config.api_key),
            Err(_) => Err("API key not configured".into()),
        }
    }

    pub fn get_download_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
        match Self::load() {
            Ok(config) => Ok(config.download_folder),
            Err(_) => Err("Download folder not configured".into()),
        }
    }
}

pub fn get_or_prompt_api_key() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(config) = Config::load() {
        if !config.api_key.is_empty() {
            return Ok(config.api_key);
        }
    }

    println!();
    println!("{}", "🔑 NoEntropy Configuration".bold().cyan());
    println!("{}", "─────────────────────────────".cyan());

    let api_key = prompt_api_key()?;

    let mut config = if let Ok(cfg) = Config::load() {
        cfg
    } else {
        Config {
            api_key: api_key.clone(),
            download_folder: PathBuf::new(),
        }
    };

    config.api_key = api_key.clone();
    config.save()?;

    println!();
    Ok(api_key)
}

pub fn get_or_prompt_download_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(config) = Config::load() {
        if !config.download_folder.as_os_str().is_empty() && config.download_folder.exists() {
            return Ok(config.download_folder);
        }
    }

    println!();
    println!("{}", "📁 Download folder not configured.".yellow());

    let folder_path = prompt_download_folder()?;

    let mut config = if let Ok(cfg) = Config::load() {
        cfg
    } else {
        Config {
            api_key: String::new(),
            download_folder: folder_path.clone(),
        }
    };

    config.download_folder = folder_path.clone();
    config.save()?;

    println!();
    Ok(folder_path)
}

fn prompt_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let mut attempts = 0;

    println!();
    println!("Get your API key at: {}", "https://ai.google.dev/".cyan().underline());
    println!("Enter your API Key (starts with 'AIza'):");

    while attempts < MAX_RETRIES {
        print!("API Key: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let key = input.trim();

        if validate_api_key(key) {
            return Ok(key.to_string());
        }

        attempts += 1;

        let remaining = MAX_RETRIES - attempts;
        eprintln!(
            "{} Invalid API key format. Must start with 'AIza' and be around 39 characters.",
            "✗".red()
        );

        if remaining > 0 {
            eprintln!("Try again ({} attempts remaining):", remaining);
        }
    }

    Err("Max retries exceeded. Please run again with a valid API key.".into())
}

fn prompt_download_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let default_path = get_default_downloads_folder();
    let default_display = default_path.to_string_lossy();

    let mut attempts = 0;

    println!(
        "Enter path to folder to organize (e.g., {}):",
        default_display.yellow()
    );
    println!("Or press Enter to use default: {}", default_display.green());
    println!("Folder path: ");

    while attempts < MAX_RETRIES {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        let path = if input.is_empty() {
            default_path.clone()
        } else {
            let expanded = expand_home(input);
            PathBuf::from(expanded)
        };

        if validate_folder_path(&path) {
            return Ok(path);
        }

        attempts += 1;

        let remaining = MAX_RETRIES - attempts;
        eprintln!("{} Invalid folder path.", "✗".red());

        if !path.exists() {
            eprintln!("  Path does not exist: {}", path.display());
        } else if !path.is_dir() {
            eprintln!("  Path is not a directory: {}", path.display());
        }

        if remaining > 0 {
            eprintln!("Try again ({} attempts remaining):", remaining);
            println!("Folder path: ");
        }
    }

    Err("Max retries exceeded. Please run again with a valid folder path.".into())
}

fn validate_api_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    if !key.starts_with("AIza") {
        return false;
    }

    if key.len() < 35 || key.len() > 50 {
        return false;
    }

    true
}

fn validate_folder_path(path: &Path) -> bool {
    if !path.exists() {
        return false;
    }

    if !path.is_dir() {
        return false;
    }

    true
}

fn get_default_downloads_folder() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        let home = base_dirs.home_dir();
        return home.join("Downloads");
    }

    PathBuf::from("./Downloads")
}

fn expand_home(path: &str) -> String {
    if path.starts_with("~/")
        && let Some(base_dirs) = BaseDirs::new()
    {
        let home = base_dirs.home_dir();
        return path.replacen("~", &home.to_string_lossy(), 1);
    }

    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(validate_api_key("AIzaSyB1234567890123456789012345678"));
        assert!(validate_api_key("AIzaSyB123456789012345678901234567890"));
    }

    #[test]
    fn test_validate_api_key_invalid() {
        assert!(!validate_api_key(""));
        assert!(!validate_api_key("invalid_key"));
        assert!(!validate_api_key("BizaSyB1234567890123456789012345678"));
        assert!(!validate_api_key("short"));
    }

    #[test]
    fn test_validate_folder_path_valid() {
        let temp_dir = tempfile::tempdir().unwrap();
        assert!(validate_folder_path(temp_dir.path()));
    }

    #[test]
    fn test_validate_folder_path_invalid() {
        assert!(!validate_folder_path(Path::new("/nonexistent/path/that/does/not/exist")));

        let temp_file = tempfile::NamedTempFile::new().unwrap();
        assert!(!validate_folder_path(temp_file.path()));
    }

    #[test]
    fn test_expand_home_with_tilde() {
        if let Some(base_dirs) = BaseDirs::new() {
            let home = base_dirs.home_dir();
            let expanded = expand_home("~/test/path");
            assert!(expanded.starts_with(home.to_string_lossy().as_ref()));
            assert!(expanded.contains("test/path"));
        }
    }

    #[test]
    fn test_expand_home_without_tilde() {
        let expanded = expand_home("/absolute/path");
        assert_eq!(expanded, "/absolute/path");

        let expanded = expand_home("relative/path");
        assert_eq!(expanded, "relative/path");
    }

    #[test]
    fn test_get_default_downloads_folder() {
        let path = get_default_downloads_folder();
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
}
