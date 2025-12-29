use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_key: String,
    pub download_folder: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err("Config file not found".into());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;

        if config.api_key.is_empty() {
            return Err("API key not found in config file".into());
        }

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;

        let toml_string = toml::to_string_pretty(self)?;

        std::fs::write(&config_path, toml_string)?;

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

    fn get_config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        match directories::ProjectDirs::from("dev", "noentropy", "NoEntropy") {
            Some(proj_dirs) => {
                let config_dir = proj_dirs.config_dir().to_path_buf();
                std::fs::create_dir_all(&config_dir)?;
                Ok(config_dir)
            }
            None => Err("Failed to determine config directory".into()),
        }
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_config_dir()?.join("config.toml"))
    }

    pub fn get_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = Self::get_config_dir()?;
        let data_dir = config_dir.join("data");
        fs::create_dir_all(&data_dir)?;
        Ok(data_dir)
    }

    pub fn get_undo_log_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_data_dir()?.join("undo_log.json"))
    }
}

pub fn get_or_prompt_api_key() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(config) = Config::load()
        && !config.api_key.is_empty()
    {
        return Ok(config.api_key);
    }

    println!();
    println!("{}", "🔑 NoEntropy Configuration".bold().cyan());
    println!("{}", "─────────────────────────────".cyan());

    let api_key = crate::prompt::Prompter::prompt_api_key()?;

    let mut config = Config::load().unwrap_or_default();
    config.api_key = api_key.clone();
    config.save()?;

    println!();
    Ok(api_key)
}

pub fn get_or_prompt_download_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(config) = Config::load()
        && !config.download_folder.as_os_str().is_empty()
        && config.download_folder.exists()
    {
        return Ok(config.download_folder);
    }

    println!();
    println!("{}", "📁 Download folder not configured.".yellow());

    let folder_path = crate::prompt::Prompter::prompt_download_folder()?;

    let mut config = Config::load().unwrap_or_default();
    config.download_folder = folder_path.clone();
    config.save()?;

    println!();
    Ok(folder_path)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            download_folder: PathBuf::new(),
        }
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
