use super::FileStorage;
use crate::{Storage, StorageError};
use astra_canvas::canvas::CanvasDocument;
use tempfile::tempdir;

#[test]
fn file_storage_save_load() {
    let dir = tempdir().unwrap();
    let storage = FileStorage::new(dir.path().to_path_buf()).unwrap();

    let mut doc = CanvasDocument::new();
    doc.name = "Test Document".to_string();

    pollster::block_on(storage.save("test-doc", &doc)).unwrap();
    let loaded = pollster::block_on(storage.load("test-doc")).unwrap();

    assert_eq!(loaded.name, "Test Document");
}

#[test]
fn file_storage_not_found() {
    let dir = tempdir().unwrap();
    let storage = FileStorage::new(dir.path().to_path_buf()).unwrap();

    let result = pollster::block_on(storage.load("nonexistent"));
    assert!(matches!(result, Err(StorageError::NotFound(_))));
}

#[test]
fn file_storage_list() {
    let dir = tempdir().unwrap();
    let storage = FileStorage::new(dir.path().to_path_buf()).unwrap();

    let doc = CanvasDocument::new();
    pollster::block_on(storage.save("doc1", &doc)).unwrap();
    pollster::block_on(storage.save("doc2", &doc)).unwrap();

    let list = pollster::block_on(storage.list()).unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.contains(&"doc1".to_string()));
    assert!(list.contains(&"doc2".to_string()));
}

#[test]
fn file_storage_delete() {
    let dir = tempdir().unwrap();
    let storage = FileStorage::new(dir.path().to_path_buf()).unwrap();

    let doc = CanvasDocument::new();
    pollster::block_on(storage.save("test", &doc)).unwrap();
    assert!(pollster::block_on(storage.exists("test")).unwrap());

    pollster::block_on(storage.delete("test")).unwrap();
    assert!(!pollster::block_on(storage.exists("test")).unwrap());
}

#[test]
fn file_storage_sanitizes_id() {
    let dir = tempdir().unwrap();
    let storage = FileStorage::new(dir.path().to_path_buf()).unwrap();

    let doc = CanvasDocument::new();
    pollster::block_on(storage.save("test/doc:with*special", &doc)).unwrap();

    let loaded = pollster::block_on(storage.load("test/doc:with*special")).unwrap();
    assert_eq!(loaded.id, doc.id);
}
