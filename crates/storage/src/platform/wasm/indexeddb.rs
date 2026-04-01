use crate::{BoxFuture, Storage, StorageError, StorageResult};
use crate::preferences::{PREFERENCES_KEY, UserPreferences};
use astra_canvas::canvas::CanvasDocument;
use rexie::{ObjectStore, Rexie, TransactionMode};
use wasm_bindgen::JsValue;

const DB_NAME: &str = "astra";
const STORE_NAME: &str = "documents";

fn other_error(message: impl std::fmt::Display) -> StorageError {
    StorageError::Other(message.to_string())
}

pub struct IndexedDbStorage;

impl IndexedDbStorage {
    pub fn new() -> Self {
        Self
    }

    async fn get_db(&self) -> StorageResult<Rexie> {
        Rexie::builder(DB_NAME)
            .version(1)
            .add_object_store(ObjectStore::new(STORE_NAME))
            .build()
            .await
            .map_err(other_error)
    }
}

impl Default for IndexedDbStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl IndexedDbStorage {
    pub async fn save_preferences(&self, prefs: &UserPreferences) -> StorageResult<()> {
        let db = self.get_db().await?;
        let transaction = db
            .transaction(&[STORE_NAME], TransactionMode::ReadWrite)
            .map_err(other_error)?;

        let store = transaction.store(STORE_NAME).map_err(other_error)?;

        let js_val = serde_wasm_bindgen::to_value(prefs)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;

        store
            .put(&js_val, Some(&JsValue::from_str(PREFERENCES_KEY)))
            .await
            .map_err(other_error)?;

        transaction.done().await.map_err(other_error)?;
        Ok(())
    }

    pub async fn load_preferences(&self) -> StorageResult<UserPreferences> {
        let db = self.get_db().await?;
        let transaction = db
            .transaction(&[STORE_NAME], TransactionMode::ReadOnly)
            .map_err(other_error)?;

        let store = transaction.store(STORE_NAME).map_err(other_error)?;

        let js_val = store
            .get(JsValue::from_str(PREFERENCES_KEY))
            .await
            .map_err(other_error)?;

        match js_val {
            Some(val) => serde_wasm_bindgen::from_value(val)
                .map_err(|e| StorageError::Serialization(e.to_string())),
            None => Ok(UserPreferences::default()),
        }
    }
}

impl Storage for IndexedDbStorage {
    fn save(&self, id: &str, document: &CanvasDocument) -> BoxFuture<'_, StorageResult<()>> {
        let id = id.to_string();
        let doc_clone = document.clone();

        Box::pin(async move {
            let db = self.get_db().await?;
            let transaction = db
                .transaction(&[STORE_NAME], TransactionMode::ReadWrite)
                .map_err(other_error)?;

            let store = transaction.store(STORE_NAME).map_err(other_error)?;

            let js_val = serde_wasm_bindgen::to_value(&doc_clone)
                .map_err(|e| StorageError::Serialization(e.to_string()))?;

            store
                .put(&js_val, Some(&JsValue::from_str(&id)))
                .await
                .map_err(other_error)?;

            transaction.done().await.map_err(other_error)?;

            Ok(())
        })
    }

    fn load(&self, id: &str) -> BoxFuture<'_, StorageResult<CanvasDocument>> {
        let id = id.to_string();

        Box::pin(async move {
            let db = self.get_db().await?;
            let transaction = db
                .transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(other_error)?;

            let store = transaction.store(STORE_NAME).map_err(other_error)?;

            let js_val = store
                .get(JsValue::from_str(&id))
                .await
                .map_err(other_error)?;

            match js_val {
                Some(val) => {
                    let mut doc: CanvasDocument = serde_wasm_bindgen::from_value(val)
                        .map_err(|e| StorageError::Serialization(e.to_string()))?;
                    doc.spatial_index.rebuild(&doc.shapes);
                    Ok(doc)
                }
                None => Err(StorageError::NotFound(id)),
            }
        })
    }

    fn delete(&self, id: &str) -> BoxFuture<'_, StorageResult<()>> {
        let id = id.to_string();

        Box::pin(async move {
            let db = self.get_db().await?;
            let transaction = db
                .transaction(&[STORE_NAME], TransactionMode::ReadWrite)
                .map_err(other_error)?;

            let store = transaction.store(STORE_NAME).map_err(other_error)?;

            store
                .delete(JsValue::from_str(&id))
                .await
                .map_err(other_error)?;

            transaction.done().await.map_err(other_error)?;

            Ok(())
        })
    }

    fn list(&self) -> BoxFuture<'_, StorageResult<Vec<String>>> {
        Box::pin(async move {
            let db = self.get_db().await?;
            let transaction = db
                .transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(other_error)?;

            let store = transaction.store(STORE_NAME).map_err(other_error)?;

            let js_keys = store
                .get_all_keys(None, None)
                .await
                .map_err(other_error)?;

            let mut ids = Vec::new();
            for key in js_keys {
                if let Some(key_str) = key.as_string() {
                    ids.push(key_str);
                }
            }
            Ok(ids)
        })
    }

    fn exists(&self, id: &str) -> BoxFuture<'_, StorageResult<bool>> {
        let id = id.to_string();

        Box::pin(async move {
            let db = self.get_db().await?;
            let transaction = db
                .transaction(&[STORE_NAME], TransactionMode::ReadOnly)
                .map_err(other_error)?;

            let store = transaction.store(STORE_NAME).map_err(other_error)?;

            let js_val = store
                .get(JsValue::from_str(&id))
                .await
                .map_err(other_error)?;

            Ok(js_val.is_some())
        })
    }
}
