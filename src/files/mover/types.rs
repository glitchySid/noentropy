use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct MoveSummary {
    moved_count: usize,
    error_count: usize,
}

impl MoveSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn moved(&mut self) {
        self.moved_count += 1;
    }

    pub fn errored(&mut self) {
        self.error_count += 1;
    }

    pub fn moved_count(&self) -> usize {
        self.moved_count
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }

    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    pub fn total_processed(&self) -> usize {
        self.moved_count + self.error_count
    }
}

#[derive(Debug)]
pub enum MoveError {
    InputReadFailed(String),
    UserCancelled,
    DirectoryCreationFailed(PathBuf, std::io::Error),
    FileMoveFailed(PathBuf, PathBuf, std::io::Error),
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::InputReadFailed(msg) => write!(f, "Failed to read input: {}", msg),
            MoveError::UserCancelled => write!(f, "Operation cancelled by user"),
            MoveError::DirectoryCreationFailed(path, err) => {
                write!(f, "Failed to create directory {:?}: {}", path, err)
            }
            MoveError::FileMoveFailed(source, target, err) => {
                write!(f, "Failed to move {:?} to {:?}: {}", source, target, err)
            }
        }
    }
}

impl std::error::Error for MoveError {}
