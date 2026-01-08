use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Preview changes without moving files")]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Maximum concurrent API requests"
    )]
    pub max_concurrent: usize,

    #[arg(long, help = "Recursively searches files in subdirectory")]
    pub recursive: bool,

    #[arg(long, help = "Undo the last file organization")]
    pub undo: bool,
    #[arg(long, help = "Change api key")]
    pub change_key: bool,

    #[arg(long, help = "Use offline mode (extension-based categorization)")]
    pub offline: bool,

    /// Optional path to organize instead of the configured download folder
    ///
    /// If provided, this path will be used instead of the download folder
    /// configured in the settings. The path will be validated and normalized
    /// (resolving `.`, `..`, and symlinks) before use.
    ///
    /// Examples:
    /// - `.` or `./` for current directory
    /// - `/absolute/path/to/folder` for absolute paths
    /// - `relative/path` for paths relative to current working directory
    #[arg(help = "Path to organize (defaults to configured download folder)")]
    pub path: Option<PathBuf>,
}
