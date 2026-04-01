use super::manager::{AutoSaveManager, LAST_DOCUMENT_KEY};
use crate::MemoryStorage;
use astra_canvas::canvas::CanvasDocument;
use std::sync::Arc;

#[test]
fn autosave_manager_creation() {
    let storage = Arc::new(MemoryStorage::new());
    let manager = AutoSaveManager::new(storage);

    assert!(!manager.is_dirty());
    assert!(!manager.should_save());
}

#[test]
fn autosave_dirty_flag() {
    let storage = Arc::new(MemoryStorage::new());
    let mut manager = AutoSaveManager::new(storage);

    assert!(!manager.is_dirty());
    manager.mark_dirty();
    assert!(manager.is_dirty());

    assert!(manager.should_save());
}

#[test]
fn autosave_save_clears_dirty() {
    let storage = Arc::new(MemoryStorage::new());
    let mut manager = AutoSaveManager::new(storage);

    manager.mark_dirty();
    assert!(manager.is_dirty());

    let doc = CanvasDocument::new();
    pollster::block_on(manager.save(&doc)).unwrap();

    assert!(!manager.is_dirty());
}

#[test]
fn autosave_load_last() {
    let storage = Arc::new(MemoryStorage::new());
    let mut manager = AutoSaveManager::new(storage);

    let mut doc = CanvasDocument::new();
    doc.name = "Test Document".to_string();
    manager.mark_dirty();
    pollster::block_on(manager.save(&doc)).unwrap();

    let storage2 = manager.storage().clone();
    let mut manager2 = AutoSaveManager::new(storage2);

    let loaded = pollster::block_on(manager2.load_last()).unwrap();
    assert_eq!(loaded.name, "Test Document");
}

#[test]
fn autosave_list_excludes_special_key() {
    let storage = Arc::new(MemoryStorage::new());
    let mut manager = AutoSaveManager::new(storage);

    let doc = CanvasDocument::new();
    manager.mark_dirty();
    pollster::block_on(manager.save(&doc)).unwrap();

    let list = pollster::block_on(manager.list_documents()).unwrap();

    assert!(!list.contains(&LAST_DOCUMENT_KEY.to_string()));
}
