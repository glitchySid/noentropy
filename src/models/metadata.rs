use crate::error::Result;
use crate::models::organization::OrganizationPlan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileMetadata {
    pub size: u64,
    pub modified: u64,
}

impl FileMetadata {
    pub fn from_path(file_path: &Path) -> Result<Self> {
        let metadata = fs::metadata(file_path)?;
        let modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();

        Ok(Self {
            size: metadata.len(),
            modified,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
    pub response: OrganizationPlan,
    pub timestamp: u64,
    pub file_metadata: HashMap<String, FileMetadata>,
}
