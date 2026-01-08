use crate::files::{FileBatch, categorize_files_offline, execute_move};
use crate::models::OrganizationPlan;
use crate::storage::UndoLog;
use colored::*;
use std::collections::HashMap;
use std::path::Path;

pub fn handle_offline_organization(
    batch: FileBatch,
    target_path: &Path,
    dry_run: bool,
    undo_log: &mut UndoLog,
) -> Result<Option<OrganizationPlan>, Box<dyn std::error::Error>> {
    println!("{}", "Categorizing files by extension...".cyan());

    let result = categorize_files_offline(batch.filenames);

    if result.plan.files.is_empty() {
        println!("{}", "No files could be categorized offline.".yellow());
        print_skipped_files(&result.skipped);
        return Ok(None);
    }

    // Print categorization summary
    print_categorization_summary(&result.plan);
    print_skipped_files(&result.skipped);

    if dry_run {
        println!("{} Dry run mode - skipping file moves.", "INFO:".cyan());
    } else {
        execute_move(target_path, result.plan, Some(undo_log));
    }

    println!("{}", "Done!".green().bold());
    Ok(None)
}

fn print_categorization_summary(plan: &OrganizationPlan) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for file in &plan.files {
        *counts.entry(file.category.as_str()).or_insert(0) += 1;
    }

    println!();
    println!("{}", "Categorized files:".green());
    for (category, count) in &counts {
        println!("  {}: {} file(s)", category.cyan(), count);
    }
    println!();
}

fn print_skipped_files(skipped: &[String]) {
    if skipped.is_empty() {
        return;
    }

    println!(
        "{} {} file(s) with unknown extension:",
        "Skipped".yellow(),
        skipped.len()
    );
    for filename in skipped.iter().take(10) {
        println!("  - {}", filename);
    }
    if skipped.len() > 10 {
        println!("  ... and {} more", skipped.len() - 10);
    }
    println!();
}
