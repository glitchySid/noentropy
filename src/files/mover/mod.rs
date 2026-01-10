use crate::models::OrganizationPlan;
use crate::storage::UndoLog;
use colored::*;
use std::path::Path;

mod confirmation;
mod display;
mod execution;
mod paths;
mod types;

use confirmation::{AutoConfirm, StdinConfirmation};
use display::print_summary;

pub use types::{MoveError, MoveSummary};

pub fn execute_move(base_path: &Path, plan: OrganizationPlan, undo_log: Option<&mut UndoLog>) {
    let confirmation = StdinConfirmation;
    match execution::execute_move_with_strategy(base_path, plan, undo_log, &confirmation) {
        Ok(summary) => print_summary(&summary),
        Err(e) => {
            if matches!(e, MoveError::UserCancelled) {
                println!("\n{}", "Operation cancelled.".red());
            } else {
                eprintln!("\n{}", format!("{}", e).red());
            }
        }
    }
}

pub fn execute_move_auto(
    base_path: &Path,
    plan: OrganizationPlan,
    undo_log: Option<&mut UndoLog>,
) -> Result<MoveSummary, MoveError> {
    let confirmation = AutoConfirm;
    execution::execute_move_with_strategy(base_path, plan, undo_log, &confirmation)
}
