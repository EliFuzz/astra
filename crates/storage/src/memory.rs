use super::{BoxFuture, Storage, StorageError, StorageResult};
use astra_canvas::canvas::CanvasDocument;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Default)]
pub struct MemoryStorage {
    documents: RwLock<HashMap<String, CanvasDocument>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for MemoryStorage {
    fn save(&self, id: &str, document: &CanvasDocument) -> BoxFuture<'_, StorageResult<()>> {
        let id = id.to_string();
        let document = document.clone();
        Box::pin(async move {
            let mut docs = self
                .documents
                .write()
                .map_err(|e| StorageError::Other(format!("Lock error: {}", e)))?;
            docs.insert(id, document);
            Ok(())
        })
    }

    fn load(&self, id: &str) -> BoxFuture<'_, StorageResult<CanvasDocument>> {
        let id = id.to_string();
        Box::pin(async move {
            let docs = self
                .documents
                .read()
                .map_err(|e| StorageError::Other(format!("Lock error: {}", e)))?;
            docs.get(&id).cloned().ok_or(StorageError::NotFound(id))
        })
    }

    fn delete(&self, id: &str) -> BoxFuture<'_, StorageResult<()>> {
        let id = id.to_string();
        Box::pin(async move {
            let mut docs = self
                .documents
                .write()
                .map_err(|e| StorageError::Other(format!("Lock error: {}", e)))?;
            docs.remove(&id);
            Ok(())
        })
    }

    fn list(&self) -> BoxFuture<'_, StorageResult<Vec<String>>> {
        Box::pin(async move {
            let docs = self
                .documents
                .read()
                .map_err(|e| StorageError::Other(format!("Lock error: {}", e)))?;
            Ok(docs.keys().cloned().collect())
        })
    }

    fn exists(&self, id: &str) -> BoxFuture<'_, StorageResult<bool>> {
        let id = id.to_string();
        Box::pin(async move {
            let docs = self
                .documents
                .read()
                .map_err(|e| StorageError::Other(format!("Lock error: {}", e)))?;
            Ok(docs.contains_key(&id))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();

        pollster::block_on(storage.save("test", &doc)).unwrap();
        let loaded = pollster::block_on(storage.load("test")).unwrap();

        assert_eq!(doc.id, loaded.id);
    }

    #[test]
    fn not_found() {
        let storage = MemoryStorage::new();
        let result = pollster::block_on(storage.load("nonexistent"));

        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn exists() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();

        assert!(!pollster::block_on(storage.exists("test")).unwrap());
        pollster::block_on(storage.save("test", &doc)).unwrap();
        assert!(pollster::block_on(storage.exists("test")).unwrap());
    }

    #[test]
    fn delete() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();

        pollster::block_on(storage.save("test", &doc)).unwrap();
        pollster::block_on(storage.delete("test")).unwrap();
        assert!(!pollster::block_on(storage.exists("test")).unwrap());
    }

    #[test]
    fn list() {
        let storage = MemoryStorage::new();
        let doc = CanvasDocument::new();

        pollster::block_on(storage.save("doc1", &doc)).unwrap();
        pollster::block_on(storage.save("doc2", &doc)).unwrap();

        let list = pollster::block_on(storage.list()).unwrap();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"doc1".to_string()));
        assert!(list.contains(&"doc2".to_string()));
    }
}
