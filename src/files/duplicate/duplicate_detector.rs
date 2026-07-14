use std::{collections::HashMap, fs::File, path::PathBuf};

use blake3::Hasher;
use colored::Colorize;
use walkdir::WalkDir;

use crate::files::duplicate::{
    confirmation::ConfirmationStrategy,
    display::print_duplicate_summary,
    types::{DuplicateError, DuplicateSummary},
};
use crate::settings::get_or_prompt_download_folder;

fn compute_file_hash(path: &PathBuf) -> Result<blake3::Hash, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize())
}

pub struct DuplicateDetector {
    path: PathBuf,
}

impl DuplicateDetector {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Phase 1: Group files by size using metadata only (fast)
    fn group_by_size(&self) -> HashMap<u64, Vec<PathBuf>> {
        WalkDir::new(&self.path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|entry| {
                let size = entry.metadata().ok()?.len();
                Some((size, entry.path().to_path_buf()))
            })
            .fold(HashMap::new(), |mut map, (size, path)| {
                map.entry(size).or_default().push(path);
                map
            })
    }

    /// Find duplicates: size pre-filter + hash only candidates
    pub fn find_duplicates(&self) -> Vec<Vec<PathBuf>> {
        let size_groups = self.group_by_size();

        size_groups
            .into_values()
            .filter(|files| files.len() > 1)
            .flat_map(|files| {
                let mut hash_map: HashMap<blake3::Hash, Vec<PathBuf>> = HashMap::new();
                for path in files {
                    if let Ok(hash) = compute_file_hash(&path) {
                        hash_map.entry(hash).or_default().push(path);
                    }
                }
                hash_map
                    .into_values()
                    .filter(|g| g.len() > 1)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Delete duplicates and return summary (metadata captured before delete)
    pub fn delete_duplicates(&self) -> Result<DuplicateSummary, DuplicateError> {
        let mut summary = DuplicateSummary::new();
        let groups = self.find_duplicates();

        for group in groups {
            let to_delete = &group[1..];
            for file in to_delete {
                if let Ok(metadata) = std::fs::metadata(file) {
                    let size = metadata.len();
                    if std::fs::remove_file(file).is_ok() {
                        summary.duplicated();
                        summary.size_saved(size);
                    }
                }
            }
        }

        Ok(summary)
    }

    /// Print duplicates (legacy CLI interface)
    pub fn print_duplicates(&self) -> Result<(), DuplicateError> {
        let duplicates = self.find_duplicates();

        if duplicates.is_empty() {
            return Err(DuplicateError::NoDuplicate);
        }

        println!("Duplicate files:");
        for group in duplicates {
            for file in &group {
                println!("{}", format!("{}", file.display()).green());
            }
            println!();
        }

        Ok(())
    }
}

pub fn execute_delete_duplicates<C: ConfirmationStrategy>(
    confirmation: &C,
    _recursive: bool,
) -> Result<DuplicateSummary, DuplicateError> {
    let download_path = get_or_prompt_download_folder()?;
    let detector = DuplicateDetector::new(download_path);

    match detector.print_duplicates() {
        Ok(_) => {
            confirmation.confirm()?;
            let summary = detector.delete_duplicates()?;
            print_duplicate_summary(&summary);
            Ok(summary)
        }
        Err(e) => Err(e),
    }
}

pub fn delete_duplicates_silent(path: &std::path::Path) -> Result<DuplicateSummary, DuplicateError> {
    let detector = DuplicateDetector::new(path.to_path_buf());
    detector.delete_duplicates()
}
