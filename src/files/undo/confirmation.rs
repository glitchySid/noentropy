use super::types::UndoError;
use std::io;

pub trait ConfirmationStrategy {
    fn confirm(&self) -> Result<bool, UndoError>;
}

pub struct StdinConfirmation;

impl ConfirmationStrategy for StdinConfirmation {
    fn confirm(&self) -> Result<bool, UndoError> {
        eprint!("\nDo you want to undo these changes? [y/N]: ");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Err(UndoError::InputReadFailed(
                "Failed to read input. Undo cancelled.".to_string(),
            ));
        }

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            return Err(UndoError::UserCancelled);
        }

        Ok(true)
    }
}

pub struct AutoConfirm;

impl ConfirmationStrategy for AutoConfirm {
    fn confirm(&self) -> Result<bool, UndoError> {
        Ok(true)
    }
}
