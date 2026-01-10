use super::types::MoveError;
use std::io;

pub trait ConfirmationStrategy {
    fn confirm(&self) -> Result<bool, MoveError>;
}

pub struct StdinConfirmation;

impl ConfirmationStrategy for StdinConfirmation {
    fn confirm(&self) -> Result<bool, MoveError> {
        eprint!("\nDo you want to apply these changes? [y/N]: ");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return Err(MoveError::InputReadFailed(
                "Failed to read input. Operation cancelled.".to_string(),
            ));
        }

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            return Err(MoveError::UserCancelled);
        }

        Ok(true)
    }
}

pub struct AutoConfirm;

impl ConfirmationStrategy for AutoConfirm {
    fn confirm(&self) -> Result<bool, MoveError> {
        Ok(true)
    }
}
