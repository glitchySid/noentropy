use super::confirmation::ConfirmationStrategy;
use super::display::{display_plan, format_target_path};
use super::paths::{build_target_path, ensure_directory_exists};
use super::types::{MoveError, MoveSummary};
use crate::files::move_file_cross_platform;
use crate::models::OrganizationPlan;
use crate::storage::UndoLog;
use colored::*;
use std::fs;
use std::path::{MAIN_SEPARATOR, Path};

pub fn execute_move_with_strategy<C: ConfirmationStrategy>(
    base_path: &Path,
    plan: OrganizationPlan,
    mut undo_log: Option<&mut UndoLog>,
    confirmation: &C,
) -> Result<MoveSummary, MoveError> {
    if plan.files.is_empty() {
        println!("{}", "No files to organize.".yellow());
        return Ok(MoveSummary::new());
    }

    display_plan(&plan.files);

    confirmation.confirm()?;

    println!("\n{}", "--- MOVING FILES ---".bold().underline());

    let mut summary = MoveSummary::new();

    for item in plan.files {
        let source = base_path.join(&item.filename);
        let final_path = base_path.join(&item.category);
        let target = build_target_path(
            base_path,
            &item.category,
            &item.sub_category,
            &item.filename,
        );

        if let Err(e) = ensure_directory_exists(&final_path) {
            eprintln!("{} {}", "ERROR:".red(), e);
            summary.errored();
            continue;
        }

        match fs::metadata(&source) {
            Ok(metadata) if metadata.is_file() => {
                match move_file_cross_platform(&source, &target) {
                    Ok(_) => {
                        let target_display = format_target_path(&item.category, &item.sub_category);
                        println!(
                            "Moved: {} -> {}{}",
                            item.filename, target_display, MAIN_SEPARATOR
                        );
                        summary.moved();

                        if let Some(ref mut log) = undo_log {
                            log.record_move(source, target);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Failed to move {}: {}", "ERROR:".red(), item.filename, e);
                        summary.errored();

                        if let Some(ref mut log) = undo_log {
                            log.record_failed_move(source, target);
                        }
                    }
                }
            }
            Ok(_) => {
                eprintln!(
                    "{} Skipping {}: Not a file",
                    "WARN:".yellow(),
                    item.filename
                );
            }
            Err(_) => {
                eprintln!(
                    "{} Skipping {}: File not found",
                    "WARN:".yellow(),
                    item.filename
                );
                summary.errored();
            }
        }
    }

    Ok(summary)
}

/// Silent version for TUI - no console output
pub fn execute_move_silent(
    base_path: &Path,
    plan: OrganizationPlan,
    mut undo_log: Option<&mut UndoLog>,
) -> Result<MoveSummary, MoveError> {
    if plan.files.is_empty() {
        return Ok(MoveSummary::new());
    }

    let mut summary = MoveSummary::new();

    for item in plan.files {
        let source = base_path.join(&item.filename);
        let final_path = base_path.join(&item.category);
        let target = build_target_path(
            base_path,
            &item.category,
            &item.sub_category,
            &item.filename,
        );

        if ensure_directory_exists(&final_path).is_err() {
            summary.errored();
            continue;
        }

        match fs::metadata(&source) {
            Ok(metadata) if metadata.is_file() => {
                match move_file_cross_platform(&source, &target) {
                    Ok(_) => {
                        summary.moved();
                        if let Some(ref mut log) = undo_log {
                            log.record_move(source, target);
                        }
                    }
                    Err(_) => {
                        summary.errored();
                        if let Some(ref mut log) = undo_log {
                            log.record_failed_move(source, target);
                        }
                    }
                }
            }
            Ok(_) => {
                // Not a file, skip silently
            }
            Err(_) => {
                summary.errored();
            }
        }
    }

    Ok(summary)
}
