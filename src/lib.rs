pub mod cli;
pub mod error;
pub mod files;
pub mod gemini;
pub mod models;
pub mod settings;
pub mod storage;
pub mod tui;

pub use cli::Args;
pub use error::Result;
pub use files::{
    FileBatch, MoveError, MoveSummary, execute_move, execute_move_auto, is_text_file,
    read_file_sample, undo_moves,
};
pub use gemini::GeminiClient;
pub use gemini::GeminiError;
pub use models::{FileCategory, FileMoveRecord, MoveStatus, OrganizationPlan};
pub use settings::Config;
pub use storage::{Cache, UndoLog};
pub use tui::run_app;
