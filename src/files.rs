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
    pub fn from_path(root_path: PathBuf) -> Self {
        let mut filenames = Vec::new();
        let mut paths = Vec::new();
        for entry in WalkDir::new(&root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            if let Ok(relative_path) = entry.path().strip_prefix(&root_path) {
                filenames.push(relative_path.to_string_lossy().into_owned());
                paths.push(entry.path().to_path_buf());
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
    // ---------------------------------------------------------
    // PHASE 1: PREVIEW (Show the plan)
    // ---------------------------------------------------------
    println!("\n{}", "--- EXECUTION PLAN ---".bold().underline());

    if plan.files.is_empty() {
        println!("{}", "No files to organize.".yellow());
        return;
    }

    // Iterate by reference (&) so we don't consume the data yet
    for item in &plan.files {
        let mut target_display = format!("{}", item.category.green());
        if !item.sub_category.is_empty() {
            target_display = format!("{}/{}", target_display, item.sub_category.blue());
        }

        println!("Plan: {} -> {}/", item.filename, target_display);
    }

    // ---------------------------------------------------------
    // PHASE 2: PROMPT (Ask for permission)
    // ---------------------------------------------------------
    eprint!("\nDo you want to apply these changes? [y/N]: ");

    let mut input = String::new();
    if io::stdin()
        .read_line(&mut input)
        .is_err()
    {
        println!("\n{}", "Failed to read input. Operation cancelled.".red());
        return;
    }

    let input = input.trim().to_lowercase();

    // If input is not "y" or "yes", abort.
    if input != "y" && input != "yes" {
        println!("\n{}", "Operation cancelled.".red());
        return;
    }

    // ---------------------------------------------------------
    // PHASE 3: EXECUTION (Actually move files)
    // ---------------------------------------------------------
    println!("\n{}", "--- MOVING FILES ---".bold().underline());

    for item in plan.files {
        let source = base_path.join(&item.filename);

        // Logic: Destination / Parent Category / Sub Category
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

        // 1. Create the category/sub-category folder
        // (Only need to call this once per file path)
        if let Err(e) = fs::create_dir_all(&final_path) {
            println!(
                "{} Failed to create dir {:?}: {}",
                "ERROR:".red(),
                final_path,
                e
            );
            continue; // Skip moving this file if we can't make the folder
        }

        // 2. Move the file
        if source.exists() {
            match fs::rename(&source, &target) {
                Ok(_) => {
                    // Formatting the success message
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
                }
                Err(e) => println!("{} Failed to move {}: {}", "ERROR:".red(), item.filename, e),
            }
        } else {
            println!(
                "{} Skipping {}: File not found",
                "WARN:".yellow(),
                item.filename
            );
        }
    }

    println!("\n{}", "Organization Complete!".bold().green());
} // --- 1. Helper to check if file is likely text ---
pub fn is_text_file(path: &Path) -> bool {
    let text_extensions = [
        "txt", "md", "rs", "py", "js", "html", "css", "json", "xml", "csv",
    ];

    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return text_extensions.contains(&ext_str.to_lowercase().as_str());
        }
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
