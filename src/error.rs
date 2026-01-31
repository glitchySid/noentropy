use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerializationError(String),

    #[error("Gemini API error: {0}")]
    GeminiError(#[from] crate::gemini::GeminiError),

    #[error("Path validation error: {0}")]
    PathValidationError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Undo log error: {0}")]
    UndoLogError(String),

    #[error("File operation error: {0}")]
    FileOperationError(String),

    #[error("Duplicate detection error: {0}")]
    DuplicateError(String),

    #[error("Undo operation error: {0}")]
    UndoError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::Unknown(err)
    }
}

impl From<std::time::SystemTimeError> for AppError {
    fn from(err: std::time::SystemTimeError) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(err: toml::ser::Error) -> Self {
        AppError::TomlSerializationError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
