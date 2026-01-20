use super::types::DuplicateError;
use std::io;

pub trait ConfirmationStrategy {
    fn confirm(&self) -> Result<bool, DuplicateError>;
}

pub struct StdinConfirmation;

impl ConfirmationStrategy for StdinConfirmation {
    fn confirm(&self) -> Result<bool, DuplicateError> {
        eprint!("\nDo you want to apply these changes? [y/N]: ");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Err(DuplicateError::InputReadFailed(
                "Failed to read input. Operation cancelled.".to_string(),
            ));
        }

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            return Err(DuplicateError::UserCancelled);
        }

        Ok(true)
    }
}

pub struct AutoConfirm;

impl ConfirmationStrategy for AutoConfirm {
    fn confirm(&self) -> Result<bool, DuplicateError> {
        Ok(true)
    }
}
