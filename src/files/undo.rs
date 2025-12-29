use crate::storage::UndoLog;
use colored::*;
use std::fs;
use std::io;
use std::path::Path;

pub fn undo_moves(
    base_path: &Path,
    undo_log: &mut UndoLog,
    dry_run: bool,
) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    let completed_moves: Vec<_> = undo_log
        .get_completed_moves()
        .into_iter()
        .cloned()
        .collect();

    if completed_moves.is_empty() {
        println!("{}", "No completed moves to undo.".yellow());
        return Ok((0, 0, 0));
    }

    println!("\n{}", "--- UNDO PREVIEW ---".bold().underline());
    println!(
        "{} will restore {} files:",
        "INFO:".cyan(),
        completed_moves.len()
    );

    for record in &completed_moves {
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

    if dry_run {
        println!("\n{}", "Dry run mode - skipping undo operation.".cyan());
        return Ok((completed_moves.len(), 0, 0));
    }

    eprint!("\nDo you want to undo these changes? [y/N]: ");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("\n{}", "Failed to read input. Undo cancelled.".red());
        return Ok((0, 0, 0));
    }

    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("\n{}", "Undo cancelled.".red());
        return Ok((0, 0, 0));
    }

    println!("\n{}", "--- UNDOING MOVES ---".bold().underline());

    let mut restored_count = 0;
    let mut skipped_count = 0;
    let mut failed_count = 0;

    for record in completed_moves {
        let source = &record.source_path;
        let destination = &record.destination_path;

        if !destination.exists() {
            eprintln!(
                "{} File not found at destination: {}",
                "WARN:".yellow(),
                destination.display()
            );
            failed_count += 1;
            continue;
        }

        if source.exists() {
            eprintln!(
                "{} Skipping {} - source already exists",
                "WARN:".yellow(),
                source.display()
            );
            skipped_count += 1;
            continue;
        }

        match move_file_cross_platform(destination, source) {
            Ok(_) => {
                println!(
                    "Restored: {} -> {}",
                    destination.display().to_string().red(),
                    source.display().to_string().green()
                );
                restored_count += 1;
                undo_log.mark_as_undone(destination);
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to restore {}: {}",
                    "ERROR:".red(),
                    source.display(),
                    e
                );
                failed_count += 1;
            }
        }
    }

    cleanup_empty_directories(base_path, undo_log)?;

    println!("\n{}", "UNDO COMPLETE!".bold().green());
    println!(
        "Files restored: {}, Skipped: {}, Failed: {}",
        restored_count.to_string().green(),
        skipped_count.to_string().yellow(),
        failed_count.to_string().red()
    );

    Ok((restored_count, skipped_count, failed_count))
}

fn cleanup_empty_directories(
    base_path: &Path,
    undo_log: &mut UndoLog,
) -> Result<(), Box<dyn std::error::Error>> {
    let directory_usage = undo_log.get_directory_usage(base_path);

    for dir_path in directory_usage.keys() {
        let full_path = base_path.join(dir_path);
        if full_path.is_dir()
            && let Ok(mut entries) = fs::read_dir(&full_path)
            && entries.next().is_none()
            && fs::remove_dir(&full_path).is_ok()
        {
            println!("{} Removed empty directory: {}", "INFO:".cyan(), dir_path);
        }
    }

    Ok(())
}

fn move_file_cross_platform(source: &Path, target: &Path) -> io::Result<()> {
    match fs::rename(source, target) {
        Ok(()) => Ok(()),
        Err(e) => {
            if cfg!(windows) || e.kind() == io::ErrorKind::CrossesDevices {
                fs::copy(source, target)?;
                fs::remove_file(source)?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
