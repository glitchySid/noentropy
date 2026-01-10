use super::cleanup::cleanup_empty_directories;
use super::confirmation::ConfirmationStrategy;
use super::display::display_undo_preview;
use super::types::{UndoError, UndoSummary};
use crate::files::move_file_cross_platform;
use crate::storage::UndoLog;
use colored::*;
use std::path::Path;

pub fn undo_with_strategy<C: ConfirmationStrategy>(
    base_path: &Path,
    undo_log: &mut UndoLog,
    confirmation: &C,
    dry_run: bool,
) -> Result<UndoSummary, UndoError> {
    let completed_moves: Vec<_> = undo_log
        .get_completed_moves()
        .into_iter()
        .cloned()
        .collect();

    if completed_moves.is_empty() {
        println!("{}", "No completed moves to undo.".yellow());
        return Ok(UndoSummary::new());
    }

    display_undo_preview(&completed_moves, base_path);

    if dry_run {
        println!("\n{}", "Dry run mode - skipping undo operation.".cyan());
        return Ok(UndoSummary::new());
    }

    confirmation.confirm()?;

    println!("\n{}", "--- UNDOING MOVES ---".bold().underline());

    let mut summary = UndoSummary::new();

    for record in completed_moves {
        let source = &record.source_path;
        let destination = &record.destination_path;

        if !destination.exists() {
            eprintln!(
                "{} File not found at destination: {}",
                "WARN:".yellow(),
                destination.display()
            );
            summary.failed();
            continue;
        }

        if source.exists() {
            eprintln!(
                "{} Skipping {} - source already exists",
                "WARN:".yellow(),
                source.display()
            );
            summary.skipped();
            continue;
        }

        match move_file_cross_platform(destination, source) {
            Ok(_) => {
                println!(
                    "Restored: {} -> {}",
                    destination.display().to_string().red(),
                    source.display().to_string().green()
                );
                summary.restored();
                undo_log.mark_as_undone(destination);
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to restore {}: {}",
                    "ERROR:".red(),
                    source.display(),
                    e
                );
                summary.failed();
            }
        }
    }

    if let Err(e) = cleanup_empty_directories(base_path, undo_log) {
        eprintln!("{} Failed to cleanup directories: {}", "WARN:".yellow(), e);
    }

    Ok(summary)
}
