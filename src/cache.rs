use crate::files::OrganizationPlan;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FileMetadata {
    size: u64,
    modified: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CacheEntry {
    pub response: OrganizationPlan,
    pub timestamp: u64,
    pub file_metadata: HashMap<String, FileMetadata>,
}

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
        let modified = metadata
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();

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

        self.entries.retain(|_, entry| {
            current_time - entry.timestamp < max_age_seconds
        });

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::FileCategory;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_cache_new() {
        let cache = Cache::new();
        assert_eq!(cache.max_entries, 1000);
        assert_eq!(cache.entries.len(), 0);
    }

    #[test]
    fn test_cache_with_max_entries() {
        let cache = Cache::with_max_entries(5);
        assert_eq!(cache.max_entries, 5);
    }

    #[test]
    fn test_cache_default() {
        let cache = Cache::default();
        assert_eq!(cache.max_entries, 1000);
    }

    #[test]
    fn test_cache_response_and_retrieve() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path();

        let mut cache = Cache::new();
        let filenames = vec!["file1.txt".to_string(), "file2.txt".to_string()];

        for filename in &filenames {
            let file_path = base_path.join(filename);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"test content").unwrap();
        }

        let response = OrganizationPlan {
            files: vec![FileCategory {
                filename: "file1.txt".to_string(),
                category: "Documents".to_string(),
                sub_category: "Text".to_string(),
            }],
        };

        cache.cache_response(&filenames, response.clone(), base_path);

        let cached = cache.get_cached_response(&filenames, base_path);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().files[0].category, "Documents");
    }

    #[test]
    fn test_cache_response_file_changed() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path();

        let mut cache = Cache::new();
        let filenames = vec!["file1.txt".to_string()];

        let file_path = base_path.join("file1.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"original content").unwrap();

        let response = OrganizationPlan {
            files: vec![FileCategory {
                filename: "file1.txt".to_string(),
                category: "Documents".to_string(),
                sub_category: "Text".to_string(),
            }],
        };

        cache.cache_response(&filenames, response.clone(), base_path);

        std::thread::sleep(std::time::Duration::from_millis(100));

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"modified content longer than original").unwrap();

        let cached = cache.get_cached_response(&filenames, base_path);
        assert!(cached.is_none());
    }

    #[test]
    fn test_cache_save_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_path = temp_dir.path().join("cache.json");
        let base_path = temp_dir.path();

        let mut cache = Cache::new();
        let filenames = vec!["file1.txt".to_string()];

        let file_path = base_path.join("file1.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test").unwrap();

        let response = OrganizationPlan {
            files: vec![FileCategory {
                filename: "file1.txt".to_string(),
                category: "Documents".to_string(),
                sub_category: "Text".to_string(),
            }],
        };

        cache.cache_response(&filenames, response, base_path);
        cache.save(&cache_path).unwrap();

        let loaded_cache = Cache::load_or_create(&cache_path);
        assert_eq!(loaded_cache.entries.len(), 1);
    }

    #[test]
    fn test_cache_cleanup_old_entries() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path();

        let mut cache = Cache::new();
        let filenames = vec!["file1.txt".to_string()];

        let file_path = base_path.join("file1.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test").unwrap();

        let response = OrganizationPlan {
            files: vec![FileCategory {
                filename: "file1.txt".to_string(),
                category: "Documents".to_string(),
                sub_category: "Text".to_string(),
            }],
        };

        cache.cache_response(&filenames, response, base_path);

        cache.cleanup_old_entries(0);
        assert_eq!(cache.entries.len(), 0);
    }

    #[test]
    fn test_cache_max_entries_eviction() {
        let temp_dir = tempfile::tempdir().unwrap();
        let base_path = temp_dir.path();

        let mut cache = Cache::with_max_entries(2);

        for i in 1..=3 {
            let filename = format!("file{}.txt", i);
            let file_path = base_path.join(&filename);
            let mut file = File::create(&file_path).unwrap();
            file.write_all(b"test").unwrap();

            let response = OrganizationPlan {
                files: vec![FileCategory {
                    filename: filename.clone(),
                    category: "Documents".to_string(),
                    sub_category: "Text".to_string(),
                }],
            };

            cache.cache_response(&vec![filename], response, base_path);
        }

        assert_eq!(cache.entries.len(), 2);
    }

    #[test]
    fn test_cache_serialization() {
        let cache = Cache::new();
        let json = serde_json::to_string(&cache).unwrap();
        let deserialized: Cache = serde_json::from_str(&json).unwrap();
        assert_eq!(cache.max_entries, deserialized.max_entries);
    }

    #[test]
    fn test_file_metadata_equality() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let metadata1 = Cache::get_file_metadata(&file_path).unwrap();
        let metadata2 = Cache::get_file_metadata(&file_path).unwrap();

        assert_eq!(metadata1, metadata2);
    }

    #[test]
    fn test_cache_key_generation() {
        let cache = Cache::new();
        let filenames1 = vec!["a.txt".to_string(), "b.txt".to_string()];
        let filenames2 = vec!["b.txt".to_string(), "a.txt".to_string()];
        let filenames3 = vec!["c.txt".to_string()];

        let key1 = cache.generate_cache_key(&filenames1);
        let key2 = cache.generate_cache_key(&filenames2);
        let key3 = cache.generate_cache_key(&filenames3);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
