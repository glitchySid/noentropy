use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileBatch {
    pub filenames: Vec<String>,
    pub paths: Vec<PathBuf>,
}

impl FileBatch {
    pub fn from_path(root_path: PathBuf, recursive: bool) -> Self {
        let mut filenames = Vec::new();
        let mut paths = Vec::new();
        let walker = if recursive {
            WalkDir::new(&root_path).min_depth(1).follow_links(false)
        } else {
            WalkDir::new(&root_path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(false)
        };
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                match path.strip_prefix(&root_path) {
                    Ok(relative_path) => {
                        filenames.push(relative_path.to_string_lossy().into_owned());
                        paths.push(path.to_path_buf());
                    }
                    Err(e) => {
                        eprintln!("Error getting relative path for {:?}: {}", path, e);
                    }
                }
            }
        }
        FileBatch { filenames, paths }
    }

    pub fn count(&self) -> usize {
        self.filenames.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::Path;

    #[test]
    fn test_file_batch_from_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.rs")).unwrap();
        fs::create_dir(dir_path.join("subdir")).unwrap();

        let batch = FileBatch::from_path(dir_path.to_path_buf(), false);
        assert_eq!(batch.count(), 2);
        assert!(batch.filenames.contains(&"file1.txt".to_string()));
        assert!(batch.filenames.contains(&"file2.rs".to_string()));
    }

    #[test]
    fn test_file_batch_from_path_nonexistent() {
        let batch = FileBatch::from_path(PathBuf::from("/nonexistent/path"), false);
        assert_eq!(batch.count(), 0);
    }

    #[test]
    fn test_file_batch_from_path_non_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();
        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.rs")).unwrap();
        fs::create_dir(dir_path.join("subdir")).unwrap();
        File::create(dir_path.join("subdir").join("file3.txt")).unwrap();
        let batch = FileBatch::from_path(dir_path.to_path_buf(), false);
        assert_eq!(batch.count(), 2);
        assert!(batch.filenames.contains(&"file1.txt".to_string()));
        assert!(batch.filenames.contains(&"file2.rs".to_string()));
        assert!(!batch.filenames.contains(&"subdir/file3.txt".to_string()));
    }

    #[test]
    fn test_file_batch_from_path_recursive() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();
        File::create(dir_path.join("file1.txt")).unwrap();
        fs::create_dir(dir_path.join("subdir1")).unwrap();
        File::create(dir_path.join("subdir1").join("file2.rs")).unwrap();
        fs::create_dir(dir_path.join("subdir1").join("nested")).unwrap();
        File::create(dir_path.join("subdir1").join("nested").join("file3.md")).unwrap();
        fs::create_dir(dir_path.join("subdir2")).unwrap();
        File::create(dir_path.join("subdir2").join("file4.py")).unwrap();
        let batch = FileBatch::from_path(dir_path.to_path_buf(), true);
        assert_eq!(batch.count(), 4);
        assert!(batch.filenames.contains(&"file1.txt".to_string()));
        assert!(batch.filenames.contains(&"subdir1/file2.rs".to_string()));
        assert!(
            batch
                .filenames
                .contains(&"subdir1/nested/file3.md".to_string())
        );
        assert!(batch.filenames.contains(&"subdir2/file4.py".to_string()));
    }
}
