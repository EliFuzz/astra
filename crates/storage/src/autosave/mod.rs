mod factory;
mod manager;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

pub use factory::{
    PlatformAutoSaveManager, PlatformStorage, create_autosave_manager, create_default_storage,
};
pub use manager::{AutoSaveManager, DEFAULT_AUTOSAVE_INTERVAL_SECS, LAST_DOCUMENT_KEY};
