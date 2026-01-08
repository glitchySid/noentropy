use colored::*;
use directories::BaseDirs;
use std::io::Write;
use std::path::{Path, PathBuf};

const MAX_RETRIES: u32 = 3;

pub struct Prompter;

impl Prompter {
    pub fn prompt_offline_mode(error_msg: &str) -> bool {
        println!();
        println!(
            "{} Unable to connect to Gemini API: {}",
            "WARNING:".yellow(),
            error_msg
        );
        println!();
        println!(
            "Continue with {} (extension-based categorization)?",
            "offline mode".cyan()
        );
        println!("Note: Files with unknown extensions will be skipped.");
        print!("[y/N]: ");

        if std::io::stdout().flush().is_err() {
            return false;
        }

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            return false;
        }

        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }

    pub fn prompt_api_key() -> Result<String, Box<dyn std::error::Error>> {
        println!();
        println!(
            "Get your API key at: {}",
            "https://ai.google.dev/".cyan().underline()
        );
        println!("Enter your API Key (starts with 'AIza'):");

        let mut attempts = 0;

        while attempts < MAX_RETRIES {
            print!("API Key: ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let key = input.trim();

            if Self::validate_api_key(key) {
                return Ok(key.to_string());
            }

            attempts += 1;
            Self::print_validation_error(
                "Invalid API key format. Must start with 'AIza' and be around 39 characters.",
                attempts,
            );
        }

        Err("Max retries exceeded. Please run again with a valid API key.".into())
    }

    pub fn prompt_download_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let default_path = Self::get_default_downloads_folder();
        let default_display = default_path.to_string_lossy();

        println!();
        println!(
            "Enter path to folder to organize (e.g., {}):",
            default_display.yellow()
        );
        println!("Or press Enter to use default: {}", default_display.green());
        println!("Folder path: ");

        let mut attempts = 0;

        while attempts < MAX_RETRIES {
            print!("> ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let input = input.trim();

            let path = if input.is_empty() {
                default_path.clone()
            } else {
                let expanded = Self::expand_home(input);
                PathBuf::from(expanded)
            };

            if Self::validate_folder_path(&path) {
                return Ok(path);
            }

            attempts += 1;
            Self::print_path_validation_error(&path, attempts);
        }

        Err("Max retries exceeded. Please run again with a valid folder path.".into())
    }

    fn print_validation_error(message: &str, attempts: u32) {
        let remaining = MAX_RETRIES - attempts;
        eprintln!("{} {}", "✗".red(), message);

        if remaining > 0 {
            eprintln!("Try again ({} attempts remaining):", remaining);
        }
    }

    fn print_path_validation_error(path: &Path, attempts: u32) {
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

    pub fn validate_api_key(key: &str) -> bool {
        !key.is_empty() && key.starts_with("AIza") && key.len() >= 35 && key.len() <= 50
    }

    pub fn validate_folder_path(path: &Path) -> bool {
        path.exists() && path.is_dir()
    }

    pub fn get_default_downloads_folder() -> PathBuf {
        BaseDirs::new()
            .map(|base_dirs| base_dirs.home_dir().join("Downloads"))
            .unwrap_or_else(|| PathBuf::from("./Downloads"))
    }

    pub fn expand_home(path: &str) -> String {
        if path.starts_with("~/")
            && let Some(base_dirs) = BaseDirs::new()
        {
            let home = base_dirs.home_dir();
            return path.replacen("~", &home.to_string_lossy(), 1);
        }
        path.to_string()
    }
}
