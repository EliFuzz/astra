#[cfg(not(target_arch = "wasm32"))]
mod native {
    use astra_canvas::canvas::CanvasDocument;
    use astra_storage::{MemoryStorage, Storage, StorageError};

    #[test]
    fn save_and_load_returns_same_document_id() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();
        let doc_id = doc.id.clone();
        pollster::block_on(storage.save("key", &doc)).unwrap();
        let loaded = pollster::block_on(storage.load("key")).unwrap();
        assert_eq!(loaded.id, doc_id);
    }

    #[test]
    fn load_missing_key_returns_not_found_error() {
        let storage = MemoryStorage::new();
        let result = pollster::block_on(storage.load("missing"));
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn exists_returns_false_before_save() {
        let storage = MemoryStorage::new();
        assert!(!pollster::block_on(storage.exists("absent")).unwrap());
    }

    #[test]
    fn exists_returns_true_after_save() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();
        pollster::block_on(storage.save("doc", &doc)).unwrap();
        assert!(pollster::block_on(storage.exists("doc")).unwrap());
    }

    #[test]
    fn delete_removes_document() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();
        pollster::block_on(storage.save("doc", &doc)).unwrap();
        pollster::block_on(storage.delete("doc")).unwrap();
        assert!(!pollster::block_on(storage.exists("doc")).unwrap());
    }

    #[test]
    fn delete_nonexistent_key_is_ok() {
        let storage = MemoryStorage::new();
        assert!(pollster::block_on(storage.delete("ghost")).is_ok());
    }

    #[test]
    fn list_returns_all_saved_keys() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();
        pollster::block_on(storage.save("a", &doc)).unwrap();
        pollster::block_on(storage.save("b", &doc)).unwrap();
        let mut keys = pollster::block_on(storage.list()).unwrap();
        keys.sort();
        assert_eq!(keys, vec!["a", "b"]);
    }

    #[test]
    fn list_on_empty_storage_returns_empty_vec() {
        let storage = MemoryStorage::new();
        let keys = pollster::block_on(storage.list()).unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn overwrite_save_replaces_existing_document() {
        let storage = MemoryStorage::new();
        let doc1 = CanvasDocument::new();
        let doc2 = CanvasDocument::new();
        let id2 = doc2.id.clone();
        pollster::block_on(storage.save("key", &doc1)).unwrap();
        pollster::block_on(storage.save("key", &doc2)).unwrap();
        let loaded = pollster::block_on(storage.load("key")).unwrap();
        assert_eq!(loaded.id, id2);
    }

    #[test]
    fn multiple_documents_are_stored_independently() {
        let storage = MemoryStorage::new();
        let doc_a = CanvasDocument::new();
        let doc_b = CanvasDocument::new();
        let id_a = doc_a.id.clone();
        let id_b = doc_b.id.clone();
        pollster::block_on(storage.save("a", &doc_a)).unwrap();
        pollster::block_on(storage.save("b", &doc_b)).unwrap();
        assert_eq!(pollster::block_on(storage.load("a")).unwrap().id, id_a);
        assert_eq!(pollster::block_on(storage.load("b")).unwrap().id, id_b);
    }
}
