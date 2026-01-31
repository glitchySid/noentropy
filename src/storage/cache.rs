use crate::error::Result;
use crate::models::{CacheEntry, FileMetadata, OrganizationPlan};
use blake3::Hasher;
use serde::{Deserialize, Serialize};
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

    pub fn load_or_create(cache_path: &Path, silent: bool) -> Self {
        if !cache_path.exists() {
            return Self::new();
        }

        match fs::read_to_string(cache_path) {
            Ok(content) => match serde_json::from_str::<Cache>(&content) {
                Ok(cache) => {
                    if !silent {
                        println!("Loaded cache with {} entries", cache.entries.len());
                    }
                    cache
                }
                Err(_) => {
                    if !silent {
                        println!("Cache corrupted, creating new cache");
                    }
                    Self::new()
                }
            },
            Err(_) => {
                if !silent {
                    println!("Failed to read cache, creating new cache");
                }
                Self::new()
            }
        }
    }

    pub fn save(&self, cache_path: &Path) -> Result<()> {
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(cache_path, content)?;
        Ok(())
    }

    pub fn check_cache(&self, filenames: &[String], base_path: &Path) -> Option<OrganizationPlan> {
        let cache_key = Self::generate_cache_key(filenames);
        let entry = self.entries.get(&cache_key)?;

        let all_unchanged = filenames.iter().all(|filename| {
            let file_path = base_path.join(filename);
            FileMetadata::from_path(&file_path).ok().as_ref() == entry.file_metadata.get(filename)
        });

        if all_unchanged {
            println!("Using cached response (timestamp: {})", entry.timestamp);
            Some(entry.response.clone())
        } else {
            None
        }
    }

    pub fn cache_response(
        &mut self,
        filenames: &[String],
        response: OrganizationPlan,
        base_path: &Path,
    ) {
        let cache_key = Self::generate_cache_key(filenames);

        let file_metadata: HashMap<String, FileMetadata> = filenames
            .iter()
            .filter_map(|filename| {
                let file_path = base_path.join(filename);
                FileMetadata::from_path(&file_path)
                    .ok()
                    .map(|m| (filename.clone(), m))
            })
            .collect();

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

    fn generate_cache_key(filenames: &[String]) -> String {
        let mut hasher = Hasher::new();

        // Only sort if we have multiple files to ensure consistent ordering
        if filenames.len() > 1 {
            let mut sorted: Vec<_> = filenames.iter().collect();
            sorted.sort();

            for filename in sorted {
                hasher.update(filename.as_bytes());
                hasher.update(b"|");
            }
        } else if let Some(filename) = filenames.first() {
            hasher.update(filename.as_bytes());
        }

        hasher.finalize().to_hex().to_string()
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

        while self.entries.len() > self.max_entries {
            self.evict_oldest();
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

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
