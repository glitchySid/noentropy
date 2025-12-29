use super::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_undo_log_creation() {
    let log = UndoLog::new();
    assert_eq!(log.get_completed_count(), 0);
    assert!(!log.has_completed_moves());
}

#[test]
fn test_record_move() {
    let mut log = UndoLog::new();
    let source = PathBuf::from("/test/source.txt");
    let dest = PathBuf::from("/test/dest/source.txt");

    log.record_move(source.clone(), dest.clone());

    assert_eq!(log.get_completed_count(), 1);
    assert!(log.has_completed_moves());

    let completed = log.get_completed_moves();
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].source_path, source);
    assert_eq!(completed[0].destination_path, dest);
    assert_eq!(completed[0].status, MoveStatus::Completed);
}

#[test]
fn test_record_failed_move() {
    let mut log = UndoLog::new();
    let source = PathBuf::from("/test/source.txt");
    let dest = PathBuf::from("/test/dest/source.txt");

    log.record_failed_move(source.clone(), dest.clone());

    assert_eq!(log.get_completed_count(), 0);
    assert!(!log.has_completed_moves());
}

#[test]
fn test_mark_as_undone() {
    let mut log = UndoLog::new();
    let source = PathBuf::from("/test/source.txt");
    let dest = PathBuf::from("/test/dest/source.txt");

    log.record_move(source.clone(), dest.clone());
    assert_eq!(log.get_completed_count(), 1);

    log.mark_as_undone(&dest);
    assert_eq!(log.get_completed_count(), 0);
}

#[test]
fn test_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let undo_log_path = temp_dir.path().join("undo_log.json");

    let mut log = UndoLog::new();
    log.record_move(
        PathBuf::from("/test/source.txt"),
        PathBuf::from("/test/dest/source.txt"),
    );

    log.save(&undo_log_path).unwrap();
    assert!(undo_log_path.exists());

    let loaded_log = UndoLog::load_or_create(&undo_log_path);
    assert_eq!(loaded_log.get_completed_count(), 1);
}

#[test]
fn test_cleanup_old_entries() {
    let mut log = UndoLog::new();

    let old_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - (10 * 24 * 60 * 60);

    let source = PathBuf::from("/test/source.txt");
    let dest = PathBuf::from("/test/dest/source.txt");

    let old_record = FileMoveRecord {
        source_path: source.clone(),
        destination_path: dest.clone(),
        timestamp: old_timestamp,
        status: MoveStatus::Undone,
    };

    log.entries.push(old_record.clone());
    log.record_move(source.clone(), dest);

    assert_eq!(log.entries.len(), 2);

    log.cleanup_old_entries(7 * 24 * 60 * 60);

    assert_eq!(log.entries.len(), 1);
    assert_eq!(log.get_completed_count(), 1);
}

#[test]
fn test_evict_oldest() {
    let mut log = UndoLog::with_max_entries(2);

    log.record_move(
        PathBuf::from("/test/source1.txt"),
        PathBuf::from("/test/dest/source1.txt"),
    );

    std::thread::sleep(std::time::Duration::from_millis(10));

    log.record_move(
        PathBuf::from("/test/source2.txt"),
        PathBuf::from("/test/dest/source2.txt"),
    );

    log.record_move(
        PathBuf::from("/test/source3.txt"),
        PathBuf::from("/test/dest/source3.txt"),
    );

    assert_eq!(log.get_completed_count(), 2);
}

#[test]
fn test_get_directory_usage() {
    let mut log = UndoLog::new();
    let base_path = PathBuf::from("/test");

    log.record_move(
        PathBuf::from("/test/source1.txt"),
        PathBuf::from("/test/Documents/report.txt"),
    );

    log.record_move(
        PathBuf::from("/test/source2.txt"),
        PathBuf::from("/test/Documents/notes.txt"),
    );

    log.record_move(
        PathBuf::from("/test/source3.txt"),
        PathBuf::from("/test/Images/photo.png"),
    );

    let usage = log.get_directory_usage(&base_path);

    assert_eq!(usage.get("Documents"), Some(&2));
    assert_eq!(usage.get("Images"), Some(&1));
}

#[test]
fn test_load_corrupted_log() {
    let temp_dir = TempDir::new().unwrap();
    let undo_log_path = temp_dir.path().join("undo_log.json");

    fs::write(&undo_log_path, "invalid json").unwrap();

    let log = UndoLog::load_or_create(&undo_log_path);
    assert_eq!(log.get_completed_count(), 0);
}

#[test]
fn test_multiple_moves_same_file() {
    let mut log = UndoLog::new();
    let source = PathBuf::from("/test/source.txt");
    let dest1 = PathBuf::from("/test/dest1/source.txt");
    let dest2 = PathBuf::from("/test/dest2/source.txt");

    log.record_move(source.clone(), dest1.clone());
    log.record_move(source.clone(), dest2);

    assert_eq!(log.get_completed_count(), 2);

    log.mark_as_undone(&dest1);
    assert_eq!(log.get_completed_count(), 1);
}
