#[derive(Debug, Clone, Default)]
pub struct DuplicateSummary {
    pub total_duplicates: u64,
    pub total_size_saved: u64,
    pub error_count: u64,
}

impl DuplicateSummary {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn duplicated(&mut self) {
        self.total_duplicates += 1;
    }
    pub fn size_saved(&mut self, size: u64) {
        self.total_size_saved += size;
    }
    pub fn errored(&mut self) {
        self.error_count += 1;
    }
    pub fn duplicate_count(&self) -> u64 {
        self.total_duplicates
    }
    pub fn total_size_saved(&self) -> u64 {
        self.total_size_saved
    }
    pub fn error_count(&self) -> u64 {
        self.error_count
    }
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
    pub fn total_processed(&self) -> u64 {
        self.total_duplicates + self.error_count
    }
}

#[derive(Debug)]
pub enum DuplicateError {
    InputReadFailed(String),
    UserCancelled,
    IoError(std::io::Error),
    WalkdirError(String),
    NoDuplicate,
}

impl From<std::io::Error> for DuplicateError {
    fn from(err: std::io::Error) -> Self {
        DuplicateError::IoError(err)
    }
}

impl std::fmt::Display for DuplicateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DuplicateError::InputReadFailed(message) => write!(f, "InputReadFailed: {}", message),
            DuplicateError::UserCancelled => write!(f, "UserCancelled"),
            DuplicateError::IoError(err) => write!(f, "IoError: {}", err),
            DuplicateError::WalkdirError(err) => write!(f, "WalkdirError: {}", err),
            DuplicateError::NoDuplicate => write!(f, "No Duplicate Found"),
        }
    }
}

impl std::error::Error for DuplicateError {}

impl From<walkdir::Error> for DuplicateError {
    fn from(err: walkdir::Error) -> Self {
        DuplicateError::WalkdirError(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for DuplicateError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        DuplicateError::InputReadFailed(err.to_string())
    }
}
