use crate::{Storage, StorageResult};
use astra_canvas::canvas::CanvasDocument;
use astra_core::{Duration, Instant};
use std::sync::Arc;

pub const DEFAULT_AUTOSAVE_INTERVAL_SECS: u64 = 30;

pub const LAST_DOCUMENT_KEY: &str = "__last_document__";

pub struct AutoSaveManager<S: Storage> {
    storage: Arc<S>,
    interval: Duration,
    last_save: Option<Instant>,
    dirty: bool,
    current_doc_id: Option<String>,
}

impl<S: Storage> AutoSaveManager<S> {
    pub fn new(storage: Arc<S>) -> Self {
        Self {
            storage,
            interval: Duration::from_secs(DEFAULT_AUTOSAVE_INTERVAL_SECS),
            last_save: None,
            dirty: false,
            current_doc_id: None,
        }
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval;
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_document_id(&mut self, id: Option<String>) {
        self.current_doc_id = id;
    }

    pub fn document_id(&self) -> Option<&str> {
        self.current_doc_id.as_deref()
    }

    pub fn should_save(&self) -> bool {
        if !self.dirty {
            return false;
        }

        match self.last_save {
            Some(last) => last.elapsed() >= self.interval,
            None => true,
        }
    }

    pub async fn maybe_save(&mut self, document: &CanvasDocument) -> StorageResult<bool> {
        if !self.should_save() {
            return Ok(false);
        }

        self.save(document).await?;
        Ok(true)
    }

    pub async fn save(&mut self, document: &CanvasDocument) -> StorageResult<()> {
        let doc_id = self
            .current_doc_id
            .clone()
            .unwrap_or_else(|| document.id.clone());

        self.storage.save(&doc_id, document).await?;

        self.storage.save(LAST_DOCUMENT_KEY, document).await?;

        self.last_save = Some(Instant::now());
        self.dirty = false;

        Ok(())
    }

    pub async fn load(&mut self, id: &str) -> StorageResult<CanvasDocument> {
        let doc = self.storage.load(id).await?;
        self.current_doc_id = Some(id.to_string());
        self.dirty = false;
        self.last_save = Some(Instant::now());
        Ok(doc)
    }

    pub async fn load_last(&mut self) -> Option<CanvasDocument> {
        match self.storage.load(LAST_DOCUMENT_KEY).await {
            Ok(doc) => {
                self.current_doc_id = Some(doc.id.clone());
                self.dirty = false;
                self.last_save = Some(Instant::now());
                Some(doc)
            }
            Err(_) => None,
        }
    }

    pub async fn delete(&self, id: &str) -> StorageResult<()> {
        self.storage.delete(id).await
    }

    pub async fn list_documents(&self) -> StorageResult<Vec<String>> {
        let mut docs = self.storage.list().await?;
        docs.retain(|id| id != LAST_DOCUMENT_KEY);
        Ok(docs)
    }

    pub async fn exists(&self, id: &str) -> StorageResult<bool> {
        self.storage.exists(id).await
    }

    pub fn storage(&self) -> &Arc<S> {
        &self.storage
    }
}
