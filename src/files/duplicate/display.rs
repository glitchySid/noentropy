use super::types::DuplicateSummary;
use colored::*;

pub(super) fn print_duplicate_summary(summary: &DuplicateSummary) {
    println!("\n{}", "Duplicate Removal Complete!".bold().green());

    if summary.duplicate_count() > 0 || summary.error_count() > 0 {
        println!(
            "Files deleted: {}, Space saved: {}, Errors: {}",
            summary.duplicate_count().to_string().green(),
            format_size(summary.total_size_saved()).blue(),
            summary.error_count().to_string().red()
        );
    } else {
        println!("{}", "No duplicate files were deleted.".yellow());
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
