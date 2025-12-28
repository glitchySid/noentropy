use crate::cache::*;
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
