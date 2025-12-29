use colored::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::{ffi::OsStr, fs, path::Path, path::PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileCategory {
    pub filename: String,
    pub category: String,
    pub sub_category: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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

    /// Helper to get the number of files found
    pub fn count(&self) -> usize {
        self.filenames.len()
    }
}

/// Move a file with cross-platform compatibility
/// Tries rename first (fastest), falls back to copy+delete if needed (e.g., cross-filesystem on Windows)
fn move_file_cross_platform(source: &Path, target: &Path) -> io::Result<()> {
    match fs::rename(source, target) {
        Ok(()) => Ok(()),
        Err(e) => {
            if cfg!(windows) || e.kind() == io::ErrorKind::CrossesDevices {
                fs::copy(source, target)?;
                fs::remove_file(source)?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

pub fn execute_move(
    base_path: &Path,
    plan: OrganizationPlan,
    mut undo_log: Option<&mut crate::undo::UndoLog>,
) {
    println!("\n{}", "--- EXECUTION PLAN ---".bold().underline());

    if plan.files.is_empty() {
        println!("{}", "No files to organize.".yellow());
        return;
    }

    for item in &plan.files {
        let mut target_display = format!("{}", item.category.green());
        if !item.sub_category.is_empty() {
            target_display = format!("{}/{}", target_display, item.sub_category.blue());
        }

        println!("Plan: {} -> {}/", item.filename, target_display);
    }

    eprint!("\nDo you want to apply these changes? [y/N]: ");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("\n{}", "Failed to read input. Operation cancelled.".red());
        return;
    }

    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("\n{}", "Operation cancelled.".red());
        return;
    }

    println!("\n{}", "--- MOVING FILES ---".bold().underline());

    let mut moved_count = 0;
    let mut error_count = 0;

    for item in plan.files {
        let source = base_path.join(&item.filename);

        let mut final_path = base_path.join(&item.category);

        if !item.sub_category.is_empty() {
            final_path = final_path.join(&item.sub_category);
        }

        let file_name = Path::new(&item.filename)
            .file_name()
            .unwrap_or_else(|| OsStr::new(&item.filename))
            .to_string_lossy()
            .into_owned();

        let target = final_path.join(&file_name);

        if let Err(e) = fs::create_dir_all(&final_path) {
            eprintln!(
                "{} Failed to create dir {:?}: {}",
                "ERROR:".red(),
                final_path,
                e
            );
            error_count += 1;
            continue;
        }

        if let Ok(metadata) = fs::metadata(&source) {
            if metadata.is_file() {
                match move_file_cross_platform(&source, &target) {
                    Ok(_) => {
                        if item.sub_category.is_empty() {
                            println!("Moved: {} -> {}/", item.filename, item.category.green());
                        } else {
                            println!(
                                "Moved: {} -> {}/{}",
                                item.filename,
                                item.category.green(),
                                item.sub_category.blue()
                            );
                        }
                        moved_count += 1;

                        if let Some(ref mut log) = undo_log {
                            log.record_move(source, target);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Failed to move {}: {}", "ERROR:".red(), item.filename, e);
                        error_count += 1;

                        if let Some(ref mut log) = undo_log {
                            log.record_failed_move(source, target);
                        }
                    }
                }
            } else {
                eprintln!(
                    "{} Skipping {}: Not a file",
                    "WARN:".yellow(),
                    item.filename
                );
            }
        } else {
            eprintln!(
                "{} Skipping {}: File not found",
                "WARN:".yellow(),
                item.filename
            );
            error_count += 1;
        }
    }

    println!("\n{}", "Organization Complete!".bold().green());
    println!(
        "Files moved: {}, Errors: {}",
        moved_count.to_string().green(),
        error_count.to_string().red()
    );
}

pub fn undo_moves(
    base_path: &Path,
    undo_log: &mut crate::undo::UndoLog,
    dry_run: bool,
) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    let completed_moves: Vec<_> = undo_log
        .get_completed_moves()
        .into_iter()
        .cloned()
        .collect();

    if completed_moves.is_empty() {
        println!("{}", "No completed moves to undo.".yellow());
        return Ok((0, 0, 0));
    }

    println!("\n{}", "--- UNDO PREVIEW ---".bold().underline());
    println!(
        "{} will restore {} files:",
        "INFO:".cyan(),
        completed_moves.len()
    );

    for record in &completed_moves {
        if let Ok(rel_dest) = record.destination_path.strip_prefix(base_path) {
            if let Ok(rel_source) = record.source_path.strip_prefix(base_path) {
                println!(
                    "  {} -> {}",
                    rel_dest.display().to_string().red(),
                    rel_source.display().to_string().green()
                );
            } else {
                println!(
                    "  {} -> {}",
                    record.destination_path.display(),
                    record.source_path.display()
                );
            }
        }
    }

    if dry_run {
        println!("\n{}", "Dry run mode - skipping undo operation.".cyan());
        return Ok((completed_moves.len(), 0, 0));
    }

    eprint!("\nDo you want to undo these changes? [y/N]: ");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("\n{}", "Failed to read input. Undo cancelled.".red());
        return Ok((0, 0, 0));
    }

    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("\n{}", "Undo cancelled.".red());
        return Ok((0, 0, 0));
    }

    println!("\n{}", "--- UNDOING MOVES ---".bold().underline());

    let mut restored_count = 0;
    let mut skipped_count = 0;
    let mut failed_count = 0;

    for record in completed_moves {
        let source = &record.source_path;
        let destination = &record.destination_path;

        if !destination.exists() {
            eprintln!(
                "{} File not found at destination: {}",
                "WARN:".yellow(),
                destination.display()
            );
            failed_count += 1;
            continue;
        }

        if source.exists() {
            eprintln!(
                "{} Skipping {} - source already exists",
                "WARN:".yellow(),
                source.display()
            );
            skipped_count += 1;
            continue;
        }

        match move_file_cross_platform(destination, source) {
            Ok(_) => {
                println!(
                    "Restored: {} -> {}",
                    destination.display().to_string().red(),
                    source.display().to_string().green()
                );
                restored_count += 1;
                undo_log.mark_as_undone(destination);
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to restore {}: {}",
                    "ERROR:".red(),
                    source.display(),
                    e
                );
                failed_count += 1;
            }
        }
    }

    cleanup_empty_directories(base_path, undo_log)?;

    println!("\n{}", "UNDO COMPLETE!".bold().green());
    println!(
        "Files restored: {}, Skipped: {}, Failed: {}",
        restored_count.to_string().green(),
        skipped_count.to_string().yellow(),
        failed_count.to_string().red()
    );

    Ok((restored_count, skipped_count, failed_count))
}

fn cleanup_empty_directories(
    base_path: &Path,
    undo_log: &mut crate::undo::UndoLog,
) -> Result<(), Box<dyn std::error::Error>> {
    let directory_usage = undo_log.get_directory_usage(base_path);

    for dir_path in directory_usage.keys() {
        let full_path = base_path.join(dir_path);
        if full_path.is_dir()
            && let Ok(mut entries) = fs::read_dir(&full_path)
            && entries.next().is_none()
            && fs::remove_dir(&full_path).is_ok()
        {
            println!("{} Removed empty directory: {}", "INFO:".cyan(), dir_path);
        }
    }

    Ok(())
}

pub fn is_text_file(path: &Path) -> bool {
    let text_extensions = [
        "txt", "md", "rs", "py", "js", "ts", "jsx", "tsx", "html", "css", "json", "xml", "csv",
        "yaml", "yml", "toml", "ini", "cfg", "conf", "log", "sh", "bat", "ps1", "sql", "c", "cpp",
        "h", "hpp", "java", "go", "rb", "php", "swift", "kt", "scala", "lua", "r", "m",
    ];

    if let Some(ext) = path.extension()
        && let Some(ext_str) = ext.to_str()
    {
        return text_extensions.contains(&ext_str.to_lowercase().as_str());
    }
    false
}

// --- 2. Helper to safely read content (with limit) ---
pub fn read_file_sample(path: &Path, max_chars: usize) -> Option<String> {
    use std::io::Read;
    // Attempt to open the file
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    // Buffer to hold file contents
    let mut buffer = Vec::new();

    // Read the whole file (or you could use take() to limit bytes read for speed)
    // For safety, let's limit the read to avoidance huge memory spikes on massive logs
    let mut handle = file.take(max_chars as u64);
    if handle.read_to_end(&mut buffer).is_err() {
        return None;
    }

    // Try to convert to UTF-8. If it fails (binary data), return None.
    String::from_utf8(buffer).ok()
}

#[cfg(test)]
#[path = "files_tests.rs"]
mod tests;
