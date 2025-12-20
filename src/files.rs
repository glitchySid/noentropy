use serde::{Deserialize, Serialize};
use std::{fs, path::Path, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileCategory {
    pub filename: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrganizationPlan {
    pub files: Vec<FileCategory>,
}

#[derive(Debug)]
pub struct FileBatch {
    pub filenames: Vec<String>,
    pub paths: Vec<PathBuf>,
}

impl FileBatch {
    /// Reads a directory path and populates lists of all files inside it.
    /// It skips sub-directories (does not read recursively).
    pub fn from_path(root_path: PathBuf) -> Self {
        let mut filenames = Vec::new();
        let mut paths = Vec::new();

        // Check if the path exists and is a directory
        if root_path.is_dir() {
            // fs::read_dir returns a Result, so we must handle it
            if let Ok(read_dir) = fs::read_dir(&root_path) {
                for child in read_dir {
                    if let Ok(child) = child {
                        // We only want to list FILES, not sub-folders,
                        // otherwise we might try to move a folder into a folder
                        if child.path().is_file() {
                            filenames.push(child.file_name().to_string_lossy().into_owned());
                            paths.push(child.path());
                        }
                    }
                }
            }
        }

        FileBatch { filenames, paths }
    }

    /// Helper to get the number of files found
    pub fn count(&self) -> usize {
        self.filenames.len()
    }
}

pub fn execute_move(base_path: &Path, plan: OrganizationPlan) {
    for item in plan.files {
        let source = base_path.join(&item.filename);
        let target_dir = base_path.join(&item.category);
        let target = target_dir.join(&item.filename);

        // 1. Create the category folder if it doesn't exist (e.g., "Downloads/Images")
        if !target_dir.exists() {
            fs::create_dir_all(&target_dir).expect("Failed to create folder");
            println!("Created folder: {:?}", item.category);
        }

        // 2. Move the file
        if source.exists() {
            match fs::rename(&source, &target) {
                Ok(_) => println!("Moved: {} -> {}/", item.filename, item.category),
                Err(e) => println!("Failed to move {}: {}", item.filename, e),
            }
        } else {
            println!("Skipping: {} (File not found)", item.filename);
        }
    }
}
