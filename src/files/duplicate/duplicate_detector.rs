use std::{collections::HashMap, fs::File, path::Path};

use blake3::Hasher;
use colored::Colorize;
use walkdir::WalkDir;

use crate::files::duplicate::{
    confirmation::ConfirmationStrategy,
    display::print_duplicate_summary,
    types::{DuplicateError, DuplicateSummary},
};
use crate::settings::get_or_prompt_download_folder;

pub fn compute_file_hash(path: &Path) -> Result<blake3::Hash, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(hasher.finalize())
}

pub fn find_duplicates<'a>(paths: &[&'a Path]) -> Vec<Vec<&'a Path>> {
    let mut hash_map: HashMap<blake3::Hash, Vec<&Path>> = HashMap::new();

    for &path in paths {
        if let Ok(hash) = compute_file_hash(path) {
            hash_map.entry(hash).or_default().push(path);
        }
    }

    hash_map
        .into_values()
        .filter(|files| files.len() > 1)
        .collect()
}

pub fn print_duplicates(path: &Path, recursive: bool) -> Result<(), DuplicateError> {
    let mut file_paths = Vec::new();

    let walker = if recursive {
        WalkDir::new(path).follow_links(false)
    } else {
        WalkDir::new(path).max_depth(1).follow_links(false)
    };

    for entry in walker.into_iter() {
        let entry = entry?;
        if entry.file_type().is_file() {
            file_paths.push(entry.path().to_path_buf());
        }
    }

    let refs: Vec<&Path> = file_paths.iter().map(|p| p.as_path()).collect();
    let duplicates = find_duplicates(&refs);

    if duplicates.is_empty() {
        return Err(DuplicateError::NoDuplicate);
    } else {
        println!("Duplicate files:");
        for group in duplicates {
            for file in group {
                println!("{}", format!("{}", file.display()).green());
            }
            println!();
        }
    }

    Ok(())
}

pub fn execute_delete_duplicates<C: ConfirmationStrategy>(
    confirmation: &C,
    recursive: bool,
) -> Result<DuplicateSummary, DuplicateError> {
    let download_path = get_or_prompt_download_folder()?;
    match print_duplicates(&download_path, recursive) {
        Ok(_) => {
            confirmation.confirm()?;

            let summary = delete_duplicates(&download_path, recursive)?;
            print_duplicate_summary(&summary);
            Ok(summary)
        }
        Err(e) => Err(e),
    }
}

pub fn delete_duplicates(path: &Path, recursive: bool) -> Result<DuplicateSummary, DuplicateError> {
    let mut file_paths = Vec::new();
    let mut summary = DuplicateSummary::new();

    let walker = if recursive {
        WalkDir::new(path).follow_links(false)
    } else {
        WalkDir::new(path).max_depth(1).follow_links(false)
    };

    for entry in walker.into_iter() {
        let entry = entry?;
        if entry.file_type().is_file() {
            file_paths.push(entry.path().to_path_buf());
        }
    }

    let refs: Vec<&Path> = file_paths.iter().map(|p| p.as_path()).collect();
    let duplicates = find_duplicates(&refs);

    if duplicates.is_empty() {
        println!("No duplicate files found to delete.");
        return Ok(summary);
    }

    let mut total_deleted = 0;

    for group in duplicates {
        if group.len() < 2 {
            continue;
        }

        // Keep the first file, delete the rest
        let to_keep = &group[0];
        let to_delete = &group[1..];

        println!("Keeping: {}", to_keep.display());

        for file in to_delete {
            match std::fs::remove_file(file) {
                Ok(_) => {
                    println!("Deleted: {}", file.display());
                    total_deleted += 1;
                    summary.duplicated();

                    if let Ok(metadata) = std::fs::metadata(file) {
                        summary.size_saved(metadata.len());
                    }
                }
                Err(e) => {
                    eprintln!("Error deleting file {}: {}", file.display(), e);
                }
            }
        }
        println!();
    }

    println!("Total files deleted: {}", total_deleted);
    Ok(summary)
}
