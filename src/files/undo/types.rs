use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct UndoSummary {
    restored_count: usize,
    skipped_count: usize,
    failed_count: usize,
}

impl UndoSummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn restored(&mut self) {
        self.restored_count += 1;
    }

    pub fn skipped(&mut self) {
        self.skipped_count += 1;
    }

    pub fn failed(&mut self) {
        self.failed_count += 1;
    }

    pub fn restored_count(&self) -> usize {
        self.restored_count
    }

    pub fn skipped_count(&self) -> usize {
        self.skipped_count
    }

    pub fn failed_count(&self) -> usize {
        self.failed_count
    }

    pub fn total_processed(&self) -> usize {
        self.restored_count + self.skipped_count + self.failed_count
    }

    pub fn has_failures(&self) -> bool {
        self.failed_count > 0
    }
}

#[derive(Debug)]
pub enum UndoError {
    InputReadFailed(String),
    UserCancelled,
    FileRestoreFailed(String, String, std::io::Error),
}

impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UndoError::InputReadFailed(msg) => write!(f, "Failed to read input: {}", msg),
            UndoError::UserCancelled => write!(f, "Undo cancelled by user"),
            UndoError::FileRestoreFailed(dest, src, err) => {
                write!(f, "Failed to restore from {} to {}: {}", dest, src, err)
            }
        }
    }
}

impl std::error::Error for UndoError {}
