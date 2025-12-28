use colored::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::{ffi::OsStr, fs, path::Path, path::PathBuf};

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

        let entries = match fs::read_dir(&root_path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error reading directory {:?}: {}", root_path, e);
                return FileBatch {
                    filenames: Vec::new(),
                    paths: Vec::new(),
                };
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file()
                && let Ok(relative_path) = path.strip_prefix(&root_path) {
                    filenames.push(relative_path.to_string_lossy().into_owned());
                    paths.push(path);
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

pub fn execute_move(base_path: &Path, plan: OrganizationPlan) {
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
    if io::stdin()
        .read_line(&mut input)
        .is_err()
    {
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
                            println!(
                                "Moved: {} -> {}/",
                                item.filename,
                                item.category.green()
                            );
                        } else {
                            println!(
                                "Moved: {} -> {}/{}",
                                item.filename,
                                item.category.green(),
                                item.sub_category.blue()
                            );
                        }
                        moved_count += 1;
                    }
                    Err(e) => {
                        eprintln!("{} Failed to move {}: {}", "ERROR:".red(), item.filename, e);
                        error_count += 1;
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
} pub fn is_text_file(path: &Path) -> bool {
    let text_extensions = [
        "txt",
        "md",
        "rs",
        "py",
        "js",
        "ts",
        "jsx",
        "tsx",
        "html",
        "css",
        "json",
        "xml",
        "csv",
        "yaml",
        "yml",
        "toml",
        "ini",
        "cfg",
        "conf",
        "log",
        "sh",
        "bat",
        "ps1",
        "sql",
        "c",
        "cpp",
        "h",
        "hpp",
        "java",
        "go",
        "rb",
        "php",
        "swift",
        "kt",
        "scala",
        "lua",
        "r",
        "m",
    ];

    if let Some(ext) = path.extension()
        && let Some(ext_str) = ext.to_str() {
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
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_is_text_file_with_text_extensions() {
        assert!(is_text_file(Path::new("test.txt")));
        assert!(is_text_file(Path::new("test.rs")));
        assert!(is_text_file(Path::new("test.py")));
        assert!(is_text_file(Path::new("test.md")));
        assert!(is_text_file(Path::new("test.json")));
    }

    #[test]
    fn test_is_text_file_with_binary_extensions() {
        assert!(!is_text_file(Path::new("test.exe")));
        assert!(!is_text_file(Path::new("test.bin")));
        assert!(!is_text_file(Path::new("test.jpg")));
        assert!(!is_text_file(Path::new("test.pdf")));
    }

    #[test]
    fn test_is_text_file_case_insensitive() {
        assert!(is_text_file(Path::new("test.TXT")));
        assert!(is_text_file(Path::new("test.RS")));
        assert!(is_text_file(Path::new("test.Py")));
    }

    #[test]
    fn test_file_batch_from_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();

        File::create(dir_path.join("file1.txt")).unwrap();
        File::create(dir_path.join("file2.rs")).unwrap();
        fs::create_dir(dir_path.join("subdir")).unwrap();

        let batch = FileBatch::from_path(dir_path.to_path_buf());
        assert_eq!(batch.count(), 2);
        assert!(batch.filenames.contains(&"file1.txt".to_string()));
        assert!(batch.filenames.contains(&"file2.rs".to_string()));
    }

    #[test]
    fn test_file_batch_from_path_nonexistent() {
        let batch = FileBatch::from_path(PathBuf::from("/nonexistent/path"));
        assert_eq!(batch.count(), 0);
    }

    #[test]
    fn test_read_file_sample() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World!").unwrap();

        let content = read_file_sample(&file_path, 1000);
        assert_eq!(content, Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_read_file_sample_with_limit() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, World! This is a long text.").unwrap();

        let content = read_file_sample(&file_path, 5);
        assert_eq!(content, Some("Hello".to_string()));
    }

    #[test]
    fn test_read_file_sample_binary_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.bin");

        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0x00, 0xFF, 0x80, 0x90]).unwrap();

        let content = read_file_sample(&file_path, 1000);
        assert_eq!(content, None);
    }

    #[test]
    fn test_read_file_sample_nonexistent() {
        let content = read_file_sample(Path::new("/nonexistent/file.txt"), 1000);
        assert_eq!(content, None);
    }

    #[test]
    fn test_organization_plan_serialization() {
        let plan = OrganizationPlan {
            files: vec![
                FileCategory {
                    filename: "test.txt".to_string(),
                    category: "Documents".to_string(),
                    sub_category: "Text".to_string(),
                },
            ],
        };

        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("test.txt"));
        assert!(json.contains("Documents"));

        let deserialized: OrganizationPlan = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.files[0].filename, "test.txt");
    }

    #[test]
    fn test_file_category_serialization() {
        let fc = FileCategory {
            filename: "file.rs".to_string(),
            category: "Code".to_string(),
            sub_category: "Rust".to_string(),
        };

        let json = serde_json::to_string(&fc).unwrap();
        let deserialized: FileCategory = serde_json::from_str(&json).unwrap();

        assert_eq!(fc.filename, deserialized.filename);
        assert_eq!(fc.category, deserialized.category);
        assert_eq!(fc.sub_category, deserialized.sub_category);
    }
}
