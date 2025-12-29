use crate::models::{CacheEntry, FileMetadata, OrganizationPlan};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    entries: HashMap<String, CacheEntry>,
    max_entries: usize,
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache {
    pub fn new() -> Self {
        Self::with_max_entries(1000)
    }

    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries,
        }
    }

    pub fn load_or_create(cache_path: &Path) -> Self {
        if cache_path.exists() {
            match fs::read_to_string(cache_path) {
                Ok(content) => match serde_json::from_str::<Cache>(&content) {
                    Ok(cache) => {
                        println!("Loaded cache with {} entries", cache.entries.len());
                        cache
                    }
                    Err(_) => {
                        println!("Cache corrupted, creating new cache");
                        Self::new()
                    }
                },
                Err(_) => {
                    println!("Failed to read cache, creating new cache");
                    Self::new()
                }
            }
        } else {
            println!("Creating new cache file");
            Self::new()
        }
    }

    pub fn save(&self, cache_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(cache_path, content)?;
        Ok(())
    }

    pub fn get_cached_response(
        &self,
        filenames: &[String],
        base_path: &Path,
    ) -> Option<OrganizationPlan> {
        let cache_key = self.generate_cache_key(filenames);

        if let Some(entry) = self.entries.get(&cache_key) {
            let mut all_files_unchanged = true;

            for filename in filenames {
                let file_path = base_path.join(filename);
                if let Ok(current_metadata) = Self::get_file_metadata(&file_path) {
                    if let Some(cached_metadata) = entry.file_metadata.get(filename) {
                        if current_metadata != *cached_metadata {
                            all_files_unchanged = false;
                            break;
                        }
                    } else {
                        all_files_unchanged = false;
                        break;
                    }
                } else {
                    all_files_unchanged = false;
                    break;
                }
            }

            if all_files_unchanged {
                println!("Using cached response (timestamp: {})", entry.timestamp);
                return Some(entry.response.clone());
            }
        }

        None
    }

    pub fn cache_response(
        &mut self,
        filenames: &[String],
        response: OrganizationPlan,
        base_path: &Path,
    ) {
        let cache_key = self.generate_cache_key(filenames);
        let mut file_metadata = HashMap::new();

        for filename in filenames {
            let file_path = base_path.join(filename);
            if let Ok(metadata) = Self::get_file_metadata(&file_path) {
                file_metadata.insert(filename.clone(), metadata);
            }
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = CacheEntry {
            response,
            timestamp,
            file_metadata,
        };

        self.entries.insert(cache_key, entry);

        if self.entries.len() > self.max_entries {
            self.evict_oldest();
        }

        println!("Cached response for {} files", filenames.len());
    }

    fn generate_cache_key(&self, filenames: &[String]) -> String {
        let mut sorted_filenames = filenames.to_vec();
        sorted_filenames.sort();

        let mut hasher = Sha256::new();
        for filename in &sorted_filenames {
            hasher.update(filename.as_bytes());
            hasher.update(b"|");
        }

        hex::encode(hasher.finalize())
    }

    fn get_file_metadata(file_path: &Path) -> Result<FileMetadata, Box<dyn std::error::Error>> {
        let metadata = fs::metadata(file_path)?;
        let modified = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs();

        Ok(FileMetadata {
            size: metadata.len(),
            modified,
        })
    }

    pub fn cleanup_old_entries(&mut self, max_age_seconds: u64) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let initial_count = self.entries.len();

        self.entries
            .retain(|_, entry| current_time - entry.timestamp < max_age_seconds);

        let removed_count = initial_count - self.entries.len();
        if removed_count > 0 {
            println!("Cleaned up {} old cache entries", removed_count);
        }

        if self.entries.len() > self.max_entries {
            self.compact_cache();
        }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.timestamp)
            .map(|(k, _)| k.clone())
        {
            self.entries.remove(&oldest_key);
            println!("Evicted oldest cache entry to maintain limit");
        }
    }

    fn compact_cache(&mut self) {
        while self.entries.len() > self.max_entries {
            self.evict_oldest();
        }
    }
}
