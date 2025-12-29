use crate::models::OrganizationPlan;
use crate::storage::UndoLog;
use colored::*;
use std::io;
use std::{ffi::OsStr, fs, path::Path};

pub fn execute_move(base_path: &Path, plan: OrganizationPlan, mut undo_log: Option<&mut UndoLog>) {
    println!("\n{}", "--- EXECUTION PLAN ---".bold().underline());

    if plan.files.is_empty() {
        println!("{}", "No files to organize.".yellow());
        return;
    }

    for item in &plan.files {
        let mut target_display = format!("{}", item.category.green());
        if !item.sub_category.is_empty() {
            target_display = format!("{}/{}", target_display, item.sub_category.blue());
        }

        println!("Plan: {} -> {}/", item.filename, target_display);
    }

    eprint!("\nDo you want to apply these changes? [y/N]: ");

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("\n{}", "Failed to read input. Operation cancelled.".red());
        return;
    }

    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("\n{}", "Operation cancelled.".red());
        return;
    }

    println!("\n{}", "--- MOVING FILES ---".bold().underline());

    let mut moved_count = 0;
    let mut error_count = 0;

    for item in plan.files {
        let source = base_path.join(&item.filename);

        let mut final_path = base_path.join(&item.category);

        if !item.sub_category.is_empty() {
            final_path = final_path.join(&item.sub_category);
        }

        let file_name = Path::new(&item.filename)
            .file_name()
            .unwrap_or_else(|| OsStr::new(&item.filename))
            .to_string_lossy()
            .into_owned();

        let target = final_path.join(&file_name);

        if let Err(e) = fs::create_dir_all(&final_path) {
            eprintln!(
                "{} Failed to create dir {:?}: {}",
                "ERROR:".red(),
                final_path,
                e
            );
            error_count += 1;
            continue;
        }

        if let Ok(metadata) = fs::metadata(&source) {
            if metadata.is_file() {
                match move_file_cross_platform(&source, &target) {
                    Ok(_) => {
                        if item.sub_category.is_empty() {
                            println!("Moved: {} -> {}/", item.filename, item.category.green());
                        } else {
                            println!(
                                "Moved: {} -> {}/{}",
                                item.filename,
                                item.category.green(),
                                item.sub_category.blue()
                            );
                        }
                        moved_count += 1;

                        if let Some(ref mut log) = undo_log {
                            log.record_move(source, target);
                        }
                    }
                    Err(e) => {
                        eprintln!("{} Failed to move {}: {}", "ERROR:".red(), item.filename, e);
                        error_count += 1;

                        if let Some(ref mut log) = undo_log {
                            log.record_failed_move(source, target);
                        }
                    }
                }
            } else {
                eprintln!(
                    "{} Skipping {}: Not a file",
                    "WARN:".yellow(),
                    item.filename
                );
            }
        } else {
            eprintln!(
                "{} Skipping {}: File not found",
                "WARN:".yellow(),
                item.filename
            );
            error_count += 1;
        }
    }

    println!("\n{}", "Organization Complete!".bold().green());
    println!(
        "Files moved: {}, Errors: {}",
        moved_count.to_string().green(),
        error_count.to_string().red()
    );
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
