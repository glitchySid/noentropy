pub mod cache;
pub mod undo_log;

pub use cache::{Cache, CacheCheckResult};
pub use undo_log::UndoLog;

#[cfg(test)]
mod tests {
    use crate::models::{FileMoveRecord, MoveStatus};
    use crate::storage::{Cache, UndoLog};
    use std::path::PathBuf;

    #[test]
    fn test_cache_new() {
        let cache = Cache::new();
        // Just verify we can create a cache
        let _ = cache;
    }

    #[test]
    fn test_cache_with_max_entries() {
        let cache = Cache::with_max_entries(100);
        let _ = cache;
    }

    #[test]
    fn test_undo_log_new() {
        let log = UndoLog::new();
        assert!(!log.has_completed_moves());
        assert_eq!(log.get_completed_count(), 0);
    }

    #[test]
    fn test_undo_log_with_max_entries() {
        let log = UndoLog::with_max_entries(500);
        assert!(!log.has_completed_moves());
        assert_eq!(log.get_completed_count(), 0);
    }

    #[test]
    fn test_undo_log_record_move() {
        let mut log = UndoLog::new();
        let source = PathBuf::from("/from/file.txt");
        let dest = PathBuf::from("/to/file.txt");

        log.record_move(source.clone(), dest.clone());

        assert!(log.has_completed_moves());
        assert_eq!(log.get_completed_count(), 1);
    }

    #[test]
    fn test_undo_log_record_failed_move() {
        let mut log = UndoLog::new();
        let source = PathBuf::from("/from/file.txt");
        let dest = PathBuf::from("/to/file.txt");

        log.record_failed_move(source.clone(), dest.clone());

        assert!(!log.has_completed_moves());
        assert_eq!(log.get_completed_count(), 0);
    }

    #[test]
    fn test_undo_log_mark_as_undone() {
        let mut log = UndoLog::new();
        let source = PathBuf::from("/from/file.txt");
        let dest = PathBuf::from("/to/file.txt");

        log.record_move(source.clone(), dest.clone());
        assert_eq!(log.get_completed_count(), 1);

        log.mark_as_undone(&dest);
        assert_eq!(log.get_completed_count(), 0);
    }

    #[test]
    fn test_file_move_record_status() {
        let record = FileMoveRecord::new(
            PathBuf::from("/from"),
            PathBuf::from("/to"),
            MoveStatus::Completed,
        );
        assert_eq!(record.status, MoveStatus::Completed);
    }

    #[test]
    fn test_get_completed_moves_empty() {
        let log: UndoLog = UndoLog::new();
        let moves = log.get_completed_moves();
        assert!(moves.is_empty());
    }

    #[test]
    fn test_get_directory_usage_empty() {
        let log: UndoLog = UndoLog::new();
        let usage = log.get_directory_usage(PathBuf::from("/").as_path());
        assert!(usage.is_empty());
    }
}
