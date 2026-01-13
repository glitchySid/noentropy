pub mod confirmation;
pub mod display;
pub mod duplicate_detector;
pub mod types;

use crate::settings::get_or_prompt_download_folder;
pub use confirmation::{AutoConfirm, ConfirmationStrategy, StdinConfirmation};
use display::print_duplicate_summary;
use duplicate_detector::{execute_delete_duplicates, print_duplicates};
pub use types::{DuplicateError, DuplicateSummary};

pub fn execute_delete(recursive: bool) {
    let confirmation = StdinConfirmation;
    match execute_delete_duplicates(&confirmation, recursive) {
        Ok(summary) => print_duplicate_summary(&summary),
        Err(err) => eprintln!("Error deleting duplicates: {}", err),
    }
}

pub fn show_duplicates(recursive: bool) {
    let download_path = match get_or_prompt_download_folder() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error getting download folder: {}", err);
            return;
        }
    };

    match print_duplicates(&download_path, recursive) {
        Ok(_) => {}
        Err(err) => eprintln!("Error finding duplicates: {}", err),
    }
}

pub fn execute_delete_auto() {
    let confirmation = AutoConfirm;
    match execute_delete_duplicates(&confirmation, false) {
        Ok(summary) => print_duplicate_summary(&summary),
        Err(err) => eprintln!("Error deleting duplicates: {}", err),
    }
}
