use std::{fs, path::Path};

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

pub fn read_file_sample(path: &Path, max_chars: usize) -> Option<String> {
    use std::io::Read;
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return None,
    };

    let mut buffer = Vec::new();
    let mut handle = file.take(max_chars as u64);
    if handle.read_to_end(&mut buffer).is_err() {
        return None;
    }

    String::from_utf8(buffer).ok()
}

#[cfg(test)]
#[path = "detector_test.rs"]
mod tests;
