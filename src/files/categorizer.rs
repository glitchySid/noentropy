use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;
use string_interner::{DefaultSymbol, StringInterner};

type Sym = DefaultSymbol;

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

fn build_extension_map() -> (HashMap<&'static str, Sym>, StringInterner) {
    let mut map = HashMap::new();
    let mut interner = StringInterner::new();

    let images_sym = interner.get_or_intern("Images");
    let documents_sym = interner.get_or_intern("Documents");
    let installers_sym = interner.get_or_intern("Installers");
    let music_sym = interner.get_or_intern("Music");
    let video_sym = interner.get_or_intern("Video");
    let archives_sym = interner.get_or_intern("Archives");
    let code_sym = interner.get_or_intern("Code");

    for &ext in IMAGE_EXTENSIONS {
        map.insert(ext, images_sym);
    }
    for &ext in DOCUMENT_EXTENSIONS {
        map.insert(ext, documents_sym);
    }
    for &ext in INSTALLER_EXTENSIONS {
        map.insert(ext, installers_sym);
    }
    for &ext in MUSIC_EXTENSIONS {
        map.insert(ext, music_sym);
    }
    for &ext in VIDEO_EXTENSIONS {
        map.insert(ext, video_sym);
    }
    for &ext in ARCHIVE_EXTENSIONS {
        map.insert(ext, archives_sym);
    }
    for &ext in CODE_EXTENSIONS {
        map.insert(ext, code_sym);
    }

    (map, interner)
}

static EXTENSION_MAP: LazyLock<(HashMap<&'static str, Sym>, StringInterner)> =
    LazyLock::new(build_extension_map);

/// Categorizes a file by its extension.
/// Returns `Some(category)` if the extension is known, `None` otherwise.
pub fn categorize_by_extension(filename: &str) -> Option<String> {
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

    EXTENSION_MAP
        .0
        .get(ext_lower.as_str())
        .map(|&sym| EXTENSION_MAP.1.resolve(sym).unwrap().to_string())
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
                    category,
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
