use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMoveRecord {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub timestamp: u64,
    pub status: MoveStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MoveStatus {
    Completed,
    Undone,
    Failed,
}

impl FileMoveRecord {
    pub fn new(source_path: PathBuf, destination_path: PathBuf, status: MoveStatus) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            source_path,
            destination_path,
            timestamp,
            status,
        }
    }
}
