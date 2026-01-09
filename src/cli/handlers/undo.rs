use crate::cli::Args;
use crate::cli::path_utils::validate_and_normalize_path;
use crate::settings::Config;
use crate::storage::UndoLog;
use colored::*;
use std::path::PathBuf;

pub async fn handle_undo(
    args: Args,
    download_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let undo_log_path = Config::get_undo_log_path()?;

    if !undo_log_path.exists() {
        println!("{}", "No undo log found. Nothing to undo.".yellow());
        return Ok(());
    }

    let mut undo_log = UndoLog::load_or_create(&undo_log_path);

    if !undo_log.has_completed_moves() {
        println!("{}", "No completed moves to undo.".yellow());
        return Ok(());
    }

    // Use custom path if provided, otherwise use the configured download path
    let target_path = args.path.unwrap_or(download_path);

    // Validate and normalize the target path early
    let target_path = match validate_and_normalize_path(&target_path).await {
        Ok(normalized) => normalized,
        Err(e) => {
            println!("{}", format!("ERROR: {}", e).red());
            return Ok(());
        }
    };

    crate::files::undo_moves(&target_path, &mut undo_log, args.dry_run)?;

    if let Err(e) = undo_log.save(&undo_log_path) {
        eprintln!(
            "{}",
            format!(
                "WARNING: Failed to save undo log to '{}': {}. Your undo history may be incomplete.",
                undo_log_path.display(),
                e
            )
            .yellow()
        );
    }

    Ok(())
}
