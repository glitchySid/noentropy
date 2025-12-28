use crate::files::OrganizationPlan;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
    pub response: OrganizationPlan,
    pub timestamp: u64,
    pub file_hashes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load_or_create(cache_path: &Path) -> Self {
        if cache_path.exists() {
            match fs::read_to_string(cache_path) {
                Ok(content) => {
                    match serde_json::from_str::<Cache>(&content) {
                        Ok(cache) => {
                            println!("Loaded cache with {} entries", cache.entries.len());
                            cache
                        }
                        Err(_) => {
                            println!("Cache corrupted, creating new cache");
                            Self::new()
                        }
                    }
                }
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

    pub fn get_cached_response(&self, filenames: &[String], base_path: &Path) -> Option<OrganizationPlan> {
        let cache_key = self.generate_cache_key(filenames);
        
        if let Some(entry) = self.entries.get(&cache_key) {
            // Check if files have changed by comparing hashes
            let mut all_files_unchanged = true;
            
            for filename in filenames {
                let file_path = base_path.join(filename);
                if let Ok(current_hash) = Self::hash_file(&file_path) {
                    if let Some(cached_hash) = entry.file_hashes.get(filename) {
                        if current_hash != *cached_hash {
                            all_files_unchanged = false;
                            break;
                        }
                    } else {
                        all_files_unchanged = false;
                        break;
                    }
                } else {
                    // File doesn't exist or can't be read
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

    pub fn cache_response(&mut self, filenames: &[String], response: OrganizationPlan, base_path: &Path) {
        let cache_key = self.generate_cache_key(filenames);
        let mut file_hashes = HashMap::new();
        
        // Hash all files for future change detection
        for filename in filenames {
            let file_path = base_path.join(filename);
            if let Ok(hash) = Self::hash_file(&file_path) {
                file_hashes.insert(filename.clone(), hash);
            }
        }
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let entry = CacheEntry {
            response,
            timestamp,
            file_hashes,
        };
        
        self.entries.insert(cache_key, entry);
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

    fn hash_file(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        if !file_path.exists() {
            return Err("File does not exist".into());
        }
        
        let mut hasher = Sha256::new();
        let content = fs::read(file_path)?;
        hasher.update(content);
        
        Ok(hex::encode(hasher.finalize()))
    }

    pub fn cleanup_old_entries(&mut self, max_age_seconds: u64) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let initial_count = self.entries.len();
        
        self.entries.retain(|_, entry| {
            current_time - entry.timestamp < max_age_seconds
        });
        
        let removed_count = initial_count - self.entries.len();
        if removed_count > 0 {
            println!("Cleaned up {} old cache entries", removed_count);
        }
    }
}