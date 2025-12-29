use clap::Parser;

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
}
