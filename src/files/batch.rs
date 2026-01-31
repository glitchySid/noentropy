use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct FileBatch {
    pub filenames: Vec<String>,
    pub paths: Vec<PathBuf>,
}

impl FileBatch {
    pub fn from_path(root_path: &Path, recursive: bool) -> Self {
        let walker = if recursive {
            WalkDir::new(root_path).min_depth(1).follow_links(false)
        } else {
            WalkDir::new(root_path)
                .min_depth(1)
                .max_depth(1)
                .follow_links(false)
        };

        let entries: Vec<_> = walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        let (filenames, paths): (Vec<_>, Vec<_>) = entries
            .into_par_iter()
            .filter_map(|entry| {
                let path = entry.path();
                path.strip_prefix(root_path).ok().map(|relative_path| {
                    (
                        relative_path.to_string_lossy().into_owned(),
                        path.to_path_buf(),
                    )
                })
            })
            .unzip();

        FileBatch { filenames, paths }
    }

    pub fn count(&self) -> usize {
        self.filenames.len()
    }
}

#[cfg(test)]
#[path = "batch_test.rs"]
mod tests;
