use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use crate::models::{FileCategory, OrganizationPlan};

static EXTENSION_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        // Images
        ("jpg", "Images"),
        ("jpeg", "Images"),
        ("png", "Images"),
        ("gif", "Images"),
        ("bmp", "Images"),
        ("svg", "Images"),
        ("webp", "Images"),
        ("ico", "Images"),
        ("tiff", "Images"),
        ("tif", "Images"),
        ("raw", "Images"),
        ("heic", "Images"),
        ("heif", "Images"),
        // Documents
        ("pdf", "Documents"),
        ("doc", "Documents"),
        ("docx", "Documents"),
        ("txt", "Documents"),
        ("rtf", "Documents"),
        ("odt", "Documents"),
        ("xls", "Documents"),
        ("xlsx", "Documents"),
        ("ppt", "Documents"),
        ("pptx", "Documents"),
        ("csv", "Documents"),
        ("md", "Documents"),
        ("epub", "Documents"),
        // Installers
        ("exe", "Installers"),
        ("msi", "Installers"),
        ("dmg", "Installers"),
        ("deb", "Installers"),
        ("rpm", "Installers"),
        ("app", "Installers"),
        ("appimage", "Installers"),
        ("pkg", "Installers"),
        ("snap", "Installers"),
        // Music
        ("mp3", "Music"),
        ("wav", "Music"),
        ("flac", "Music"),
        ("aac", "Music"),
        ("ogg", "Music"),
        ("wma", "Music"),
        ("m4a", "Music"),
        ("opus", "Music"),
        ("aiff", "Music"),
        // Video
        ("mp4", "Video"),
        ("mkv", "Video"),
        ("avi", "Video"),
        ("mov", "Video"),
        ("wmv", "Video"),
        ("flv", "Video"),
        ("webm", "Video"),
        ("m4v", "Video"),
        ("mpeg", "Video"),
        ("mpg", "Video"),
        // Archives
        ("zip", "Archives"),
        ("tar", "Archives"),
        ("gz", "Archives"),
        ("rar", "Archives"),
        ("7z", "Archives"),
        ("bz2", "Archives"),
        ("xz", "Archives"),
        ("tgz", "Archives"),
        ("zst", "Archives"),
        // Code
        ("rs", "Code"),
        ("py", "Code"),
        ("js", "Code"),
        ("ts", "Code"),
        ("java", "Code"),
        ("c", "Code"),
        ("cpp", "Code"),
        ("h", "Code"),
        ("hpp", "Code"),
        ("go", "Code"),
        ("rb", "Code"),
        ("php", "Code"),
        ("html", "Code"),
        ("css", "Code"),
        ("json", "Code"),
        ("yaml", "Code"),
        ("yml", "Code"),
        ("toml", "Code"),
        ("xml", "Code"),
        ("sh", "Code"),
        ("bash", "Code"),
        ("sql", "Code"),
    ])
});

/// Categorizes a file by its extension.
/// Returns `Some(category)` if the extension is known, `None` otherwise.
pub fn categorize_by_extension(filename: &str) -> Option<&'static str> {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .as_deref()
        .and_then(|ext| EXTENSION_MAP.get(ext).copied())
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
