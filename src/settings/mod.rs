pub mod config;
pub mod prompt;

pub use config::{Config, get_or_prompt_api_key, get_or_prompt_download_folder};
pub use prompt::Prompter;

#[cfg(test)]
mod tests;
