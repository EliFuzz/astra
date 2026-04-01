use super::manager::AutoSaveManager;
use crate::StorageResult;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
pub fn create_default_storage() -> StorageResult<Arc<crate::FileStorage>> {
    Ok(Arc::new(crate::FileStorage::default_location()?))
}

#[cfg(target_arch = "wasm32")]
pub fn create_default_storage() -> StorageResult<Arc<crate::IndexedDbStorage>> {
    Ok(Arc::new(crate::IndexedDbStorage::new()))
}

#[cfg(not(target_arch = "wasm32"))]
pub type PlatformStorage = crate::FileStorage;

#[cfg(target_arch = "wasm32")]
pub type PlatformStorage = crate::IndexedDbStorage;

pub type PlatformAutoSaveManager = AutoSaveManager<PlatformStorage>;

pub fn create_autosave_manager() -> StorageResult<PlatformAutoSaveManager> {
    let storage = create_default_storage()?;
    Ok(AutoSaveManager::new(storage))
}
