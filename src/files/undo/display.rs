use crate::models::FileMoveRecord;
use colored::*;
use std::path::Path;

pub(super) fn display_undo_preview(records: &[FileMoveRecord], base_path: &Path) {
    println!("\n{}", "--- UNDO PREVIEW ---".bold().underline());
    println!("{} will restore {} files:", "INFO:".cyan(), records.len());

    for record in records {
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
}

pub(super) fn print_undo_summary(summary: &super::types::UndoSummary) {
    println!("\n{}", "UNDO COMPLETE!".bold().green());
    println!(
        "Files restored: {}, Skipped: {}, Failed: {}",
        summary.restored_count().to_string().green(),
        summary.skipped_count().to_string().yellow(),
        summary.failed_count().to_string().red()
    );
}
