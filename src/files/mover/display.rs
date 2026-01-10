use crate::models::FileCategory;
use colored::*;
use std::path::MAIN_SEPARATOR;

pub(super) fn display_plan(files: &[FileCategory]) {
    println!("\n{}", "--- EXECUTION PLAN ---".bold().underline());

    if files.is_empty() {
        println!("{}", "No files to organize.".yellow());
        return;
    }

    for item in files {
        let target_display = format_target_path(&item.category, &item.sub_category);
        println!(
            "Plan: {} -> {}{}",
            item.filename, target_display, MAIN_SEPARATOR
        );
    }
}

pub(super) fn print_summary(summary: &super::types::MoveSummary) {
    println!("\n{}", "Organization Complete!".bold().green());
    println!(
        "Files moved: {}, Errors: {}",
        summary.moved_count().to_string().green(),
        summary.error_count().to_string().red()
    );
}

pub(super) fn format_target_path(category: &str, sub_category: &str) -> String {
    let target_display = format!("{}", category.green());
    if sub_category.is_empty() {
        target_display
    } else {
        format!(
            "{}{}{}",
            target_display,
            MAIN_SEPARATOR,
            sub_category.blue()
        )
    }
}
