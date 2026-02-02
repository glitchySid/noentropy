use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to organize (defaults to configured download folder)
    #[arg(global = true)]
    pub path: Option<PathBuf>,

    /// Preview changes without moving files
    #[arg(long, short = 'd', global = true)]
    pub dry_run: bool,

    /// Force offline mode even if online is preferred
    #[arg(long, global = true)]
    pub offline: bool,

    /// Recursively search files in subdirectory
    #[arg(long, short = 'r', global = true)]
    pub recursive: bool,

    /// Use online mode (AI categorization)
    #[arg(long, short = 'o', global = true)]
    pub online: bool,

    /// Skip AI deep inspection for sub-categorization (faster)
    #[arg(long, global = true)]
    pub skip_deep_inspect: bool,

    /// Enable AI deep inspection for sub-categorization (slower but more accurate)
    #[arg(long, global = true)]
    pub no_skip_deep_inspect: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Organize downloads (offline-first by default)
    #[command(name = "organize")]
    Organize {
        #[arg(long, help = "Preview changes without moving files")]
        dry_run: bool,
        #[arg(long, default_value_t = 5, help = "Maximum concurrent API requests")]
        max_concurrent: usize,
        #[arg(long, help = "Use online mode (AI categorization)")]
        online: bool,
        #[arg(long, help = "Force offline mode (extension-based categorization)")]
        offline: bool,
        #[arg(long, help = "Recursively search files in subdirectory")]
        recursive: bool,
        #[arg(help = "Path to organize (defaults to configured download folder)")]
        path: Option<PathBuf>,
        #[arg(long, help = "Skip AI deep inspection (faster)")]
        skip_deep_inspect: bool,
        #[arg(long, help = "Enable AI deep inspection (slower but accurate)")]
        no_skip_deep_inspect: bool,
    },
    /// Undo the last file organization
    Undo {
        #[arg(long, help = "Preview changes without moving files")]
        dry_run: bool,
        #[arg(help = "Path to undo (defaults to configured download folder)")]
        path: Option<PathBuf>,
    },
    /// Change the API key
    #[command(name = "key")]
    ChangeKey,
    /// Detect and delete duplicate files
    #[command(name = "duplicates")]
    Duplicates {
        #[arg(long, help = "Recursively search files in subdirectory")]
        recursive: bool,
    },
}
