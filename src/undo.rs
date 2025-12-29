use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UndoLog {
    entries: Vec<FileMoveRecord>,
    max_entries: usize,
}

impl Default for UndoLog {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoLog {
    pub fn new() -> Self {
        Self::with_max_entries(1000)
    }

    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn load_or_create(undo_log_path: &Path) -> Self {
        if undo_log_path.exists() {
            match fs::read_to_string(undo_log_path) {
                Ok(content) => match serde_json::from_str::<UndoLog>(&content) {
                    Ok(log) => {
                        println!("Loaded undo log with {} entries", log.get_completed_count());
                        log
                    }
                    Err(_) => {
                        println!("Undo log corrupted, creating new log");
                        Self::new()
                    }
                },
                Err(_) => {
                    println!("Failed to read undo log, creating new log");
                    Self::new()
                }
            }
        } else {
            println!("Creating new undo log file");
            Self::new()
        }
    }

    pub fn save(&self, undo_log_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = undo_log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(undo_log_path, content)?;
        Ok(())
    }

    pub fn record_move(&mut self, source_path: PathBuf, destination_path: PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let record = FileMoveRecord {
            source_path,
            destination_path,
            timestamp,
            status: MoveStatus::Completed,
        };

        self.entries.push(record);

        if self.entries.len() > self.max_entries {
            self.evict_oldest();
        }
    }

    pub fn record_failed_move(&mut self, source_path: PathBuf, destination_path: PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let record = FileMoveRecord {
            source_path,
            destination_path,
            timestamp,
            status: MoveStatus::Failed,
        };

        self.entries.push(record);

        if self.entries.len() > self.max_entries {
            self.evict_oldest();
        }
    }

    pub fn get_completed_moves(&self) -> Vec<&FileMoveRecord> {
        self.entries
            .iter()
            .filter(|entry| entry.status == MoveStatus::Completed)
            .collect()
    }

    pub fn mark_as_undone(&mut self, source_path: &Path) {
        for entry in &mut self.entries {
            if entry.status == MoveStatus::Completed && entry.destination_path == source_path {
                entry.status = MoveStatus::Undone;
                break;
            }
        }
    }

    pub fn get_completed_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.status == MoveStatus::Completed)
            .count()
    }

    pub fn has_completed_moves(&self) -> bool {
        self.get_completed_count() > 0
    }

    pub fn cleanup_old_entries(&mut self, max_age_seconds: u64) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let initial_count = self.entries.len();

        self.entries.retain(|entry| {
            current_time - entry.timestamp < max_age_seconds
                || entry.status == MoveStatus::Completed
        });

        let removed_count = initial_count - self.entries.len();
        if removed_count > 0 {
            println!("Cleaned up {} old undo log entries", removed_count);
        }

        if self.entries.len() > self.max_entries {
            self.compact_log();
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_index) = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.status == MoveStatus::Undone)
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(i, _)| i)
        {
            self.entries.remove(oldest_index);
            println!("Evicted oldest undone log entry to maintain limit");
            return;
        }

        if let Some(oldest_index) = self
            .entries
            .iter()
            .enumerate()
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(i, _)| i)
        {
            self.entries.remove(oldest_index);
            println!("Evicted oldest log entry to maintain limit");
        }
    }

    fn compact_log(&mut self) {
        while self.entries.len() > self.max_entries {
            self.evict_oldest();
        }
    }

    pub fn get_directory_usage(&self, base_path: &Path) -> HashMap<String, usize> {
        let mut usage = HashMap::new();

        for entry in &self.entries {
            if entry.status == MoveStatus::Completed
                && let Ok(rel_path) = entry.destination_path.strip_prefix(base_path)
                && let Some(parent) = rel_path.parent()
            {
                let dir_path = parent.to_string_lossy().into_owned();
                *usage.entry(dir_path).or_insert(0) += 1;
            }
        }

        usage
    }
}

#[cfg(test)]
#[path = "undo_tests.rs"]
mod tests;
