pub mod confirmation;
pub mod display;
pub mod duplicate_detector;
pub mod types;

use crate::settings::get_or_prompt_download_folder;
pub use confirmation::{AutoConfirm, ConfirmationStrategy, StdinConfirmation};
use display::print_duplicate_summary;
pub use duplicate_detector::DuplicateDetector;
pub use types::{DuplicateError, DuplicateSummary};

pub fn execute_delete(recursive: bool) {
    let confirmation = StdinConfirmation;
    match execute_delete_duplicates(&confirmation, recursive) {
        Ok(summary) => print_duplicate_summary(&summary),
        Err(err) => eprintln!("Error deleting duplicates: {}", err),
    }
}

pub fn show_duplicates(_recursive: bool) {
    let download_path = match get_or_prompt_download_folder() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error getting download folder: {}", err);
            return;
        }
    };

    let detector = DuplicateDetector::new(download_path);
    if let Err(err) = detector.print_duplicates() {
        eprintln!("Error finding duplicates: {}", err);
    }
}

pub fn execute_delete_auto() {
    let confirmation = AutoConfirm;
    match execute_delete_duplicates(&confirmation, false) {
        Ok(summary) => print_duplicate_summary(&summary),
        Err(err) => eprintln!("Error deleting duplicates: {}", err),
    }
}

pub fn execute_delete_silent() -> Result<DuplicateSummary, DuplicateError> {
    let download_path = get_or_prompt_download_folder()?;
    let detector = DuplicateDetector::new(download_path);
    detector.delete_duplicates()
}

fn execute_delete_duplicates<C: ConfirmationStrategy>(
    confirmation: &C,
    _recursive: bool,
) -> Result<DuplicateSummary, DuplicateError> {
    let download_path = get_or_prompt_download_folder()?;
    let detector = DuplicateDetector::new(download_path);

    match detector.print_duplicates() {
        Ok(_) => {
            confirmation.confirm()?;
            let summary = detector.delete_duplicates()?;
            print_duplicate_summary(&summary);
            Ok(summary)
        }
        Err(e) => Err(e),
    }
}
