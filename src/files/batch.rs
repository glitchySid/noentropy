use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileBatch {
    pub filenames: Vec<String>,
    pub paths: Vec<PathBuf>,
}

impl FileBatch {
    pub fn from_path(root_path: &Path, recursive: bool) -> Self {
        let mut filenames = Vec::new();
        let mut paths = Vec::new();
        let walker = if recursive {
            WalkDir::new(root_path).min_depth(1).follow_links(false)
        } else {
            WalkDir::new(root_path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(false)
        };
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                match path.strip_prefix(root_path) {
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
#[path = "batch_test.rs"]
mod tests;
