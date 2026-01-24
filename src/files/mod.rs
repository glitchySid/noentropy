pub mod batch;
pub mod categorizer;
pub mod detector;
pub mod duplicate;
mod file_ops;
pub mod mover;
pub mod undo;

pub use batch::FileBatch;
pub use categorizer::{OfflineCategorizationResult, categorize_files_offline};
pub use detector::{is_text_file, read_file_sample};
pub use file_ops::move_file_cross_platform;
pub use mover::{MoveError, MoveSummary, execute_move, execute_move_auto, execute_move_silent};
pub use undo::{UndoError, UndoSummary, undo_moves, undo_moves_auto};

#[cfg(test)]
mod tests {
    use crate::models::{FileCategory, OrganizationPlan};
    use serde_json;

    #[test]
    fn test_organization_plan_serialization() {
        let plan = OrganizationPlan {
            files: vec![FileCategory {
                filename: "test.txt".to_string(),
                category: "Documents".to_string(),
                sub_category: "Text".to_string(),
            }],
        };

        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("test.txt"));
        assert!(json.contains("Documents"));

        let deserialized: OrganizationPlan = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.files[0].filename, "test.txt");
    }

    #[test]
    fn test_file_category_serialization() {
        let fc = FileCategory {
            filename: "file.rs".to_string(),
            category: "Code".to_string(),
            sub_category: "Rust".to_string(),
        };

        let json = serde_json::to_string(&fc).unwrap();
        let deserialized: FileCategory = serde_json::from_str(&json).unwrap();

        assert_eq!(fc.filename, deserialized.filename);
        assert_eq!(fc.category, deserialized.category);
        assert_eq!(fc.sub_category, deserialized.sub_category);
    }
}
