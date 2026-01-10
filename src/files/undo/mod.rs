use crate::storage::UndoLog;
use colored::*;
use std::path::Path;

mod cleanup;
mod confirmation;
mod display;
mod execution;
mod types;

use confirmation::{AutoConfirm, StdinConfirmation};
use display::print_undo_summary;

pub use types::{UndoError, UndoSummary};

pub fn undo_moves(
    base_path: &Path,
    undo_log: &mut UndoLog,
    dry_run: bool,
) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    let confirmation = StdinConfirmation;
    match execution::undo_with_strategy(base_path, undo_log, &confirmation, dry_run) {
        Ok(summary) => {
            if !dry_run {
                print_undo_summary(&summary);
            }
            Ok((
                summary.restored_count(),
                summary.skipped_count(),
                summary.failed_count(),
            ))
        }
        Err(e) => {
            if matches!(e, UndoError::UserCancelled) {
                println!("\n{}", "Undo cancelled.".red());
            } else {
                eprintln!("\n{}", format!("{}", e).red());
            }
            Ok((0, 0, 0))
        }
    }
}

pub fn undo_moves_auto(
    base_path: &Path,
    undo_log: &mut UndoLog,
    dry_run: bool,
) -> Result<UndoSummary, UndoError> {
    let confirmation = AutoConfirm;
    execution::undo_with_strategy(base_path, undo_log, &confirmation, dry_run)
}
