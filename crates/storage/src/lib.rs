mod autosave;
mod memory;
mod platform;
pub mod pending;
pub mod preferences;

pub use autosave::{
    AutoSaveManager, DEFAULT_AUTOSAVE_INTERVAL_SECS, LAST_DOCUMENT_KEY, PlatformAutoSaveManager,
    PlatformStorage, create_autosave_manager, create_default_storage,
};
pub use memory::MemoryStorage;
pub use pending::*;
pub use platform::StorageBounds;
pub use preferences::UserPreferences;

#[cfg(not(target_arch = "wasm32"))]
pub use platform::native::file::FileStorage;

#[cfg(not(target_arch = "wasm32"))]
pub use platform::native::io as native_io;

#[cfg(target_arch = "wasm32")]
pub use platform::wasm::indexeddb::IndexedDbStorage;

use astra_canvas::canvas::CanvasDocument;
use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Document not found: {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Storage error: {0}")]
    Other(String),
}

pub type StorageResult<T> = Result<T, StorageError>;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub trait Storage: StorageBounds {
    fn save(&self, id: &str, document: &CanvasDocument) -> BoxFuture<'_, StorageResult<()>>;
    fn load(&self, id: &str) -> BoxFuture<'_, StorageResult<CanvasDocument>>;
    fn delete(&self, id: &str) -> BoxFuture<'_, StorageResult<()>>;
    fn list(&self) -> BoxFuture<'_, StorageResult<Vec<String>>>;
    fn exists(&self, id: &str) -> BoxFuture<'_, StorageResult<bool>>;
}
