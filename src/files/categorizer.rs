use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::models::{FileCategory, OrganizationPlan};

const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff", "tif", "raw", "heic", "heif",
];
const DOCUMENT_EXTENSIONS: &[&str] = &[
    "pdf", "doc", "docx", "txt", "rtf", "odt", "xls", "xlsx", "ppt", "pptx", "csv", "md", "epub",
];
const INSTALLER_EXTENSIONS: &[&str] = &[
    "exe", "msi", "dmg", "deb", "rpm", "app", "appimage", "pkg", "snap",
];
const MUSIC_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus", "aiff",
];
const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpeg", "mpg",
];
const ARCHIVE_EXTENSIONS: &[&str] = &["zip", "tar", "gz", "rar", "7z", "bz2", "xz", "tgz", "zst"];
const CODE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "java", "c", "cpp", "h", "hpp", "go", "rb", "php", "html", "css",
    "json", "yaml", "yml", "toml", "xml", "sh", "bash", "sql",
];

fn build_extension_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    for &ext in IMAGE_EXTENSIONS {
        map.insert(ext, "Images");
    }
    for &ext in DOCUMENT_EXTENSIONS {
        map.insert(ext, "Documents");
    }
    for &ext in INSTALLER_EXTENSIONS {
        map.insert(ext, "Installers");
    }
    for &ext in MUSIC_EXTENSIONS {
        map.insert(ext, "Music");
    }
    for &ext in VIDEO_EXTENSIONS {
        map.insert(ext, "Video");
    }
    for &ext in ARCHIVE_EXTENSIONS {
        map.insert(ext, "Archives");
    }
    for &ext in CODE_EXTENSIONS {
        map.insert(ext, "Code");
    }

    map
}

static EXTENSION_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(build_extension_map);

/// Categorizes a file by its extension.
/// Returns `Some(category)` if the extension is known, `None` otherwise.
pub fn categorize_by_extension(filename: &str) -> Option<&'static str> {
    // Early return for empty or invalid filenames
    if filename.is_empty() {
        return None;
    }

    // Use a more efficient path to get extension
    let path = Path::new(filename);
    let ext = path.extension()?.to_str()?;

    // Avoid allocation by checking if lowercase is needed
    let ext_lower = if ext.chars().any(|c| c.is_uppercase()) {
        ext.to_lowercase()
    } else {
        ext.to_string()
    };

    EXTENSION_MAP.get(ext_lower.as_str()).copied()
}

/// Result of offline categorization
pub struct OfflineCategorizationResult {
    pub plan: OrganizationPlan,
    pub skipped: Vec<String>,
}

/// Categorizes a list of filenames using extension-based rules.
/// Returns categorized files and a list of skipped filenames.
pub fn categorize_files_offline(filenames: Vec<String>) -> OfflineCategorizationResult {
    let mut files = Vec::with_capacity(filenames.len());
    let mut skipped = Vec::new();

    for filename in filenames {
        match categorize_by_extension(&filename) {
            Some(category) => {
                files.push(FileCategory {
                    filename,
                    category: category.to_string(),
                    sub_category: String::new(),
                });
            }
            None => {
                skipped.push(filename);
            }
        }
    }

    OfflineCategorizationResult {
        plan: OrganizationPlan { files },
        skipped,
    }
}

#[cfg(test)]
#[path = "categorizer_test.rs"]
mod tests;
