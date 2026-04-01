use crate::{BoxFuture, Storage, StorageError, StorageResult};
use astra_canvas::canvas::CanvasDocument;
use std::fs;
use std::path::PathBuf;

pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: PathBuf) -> StorageResult<Self> {
        if !base_path.exists() {
            fs::create_dir_all(&base_path).map_err(|e| {
                StorageError::Io(format!("Failed to create storage directory: {}", e))
            })?;
        }
        Ok(Self { base_path })
    }

    pub fn default_location() -> StorageResult<Self> {
        let base = dirs::data_local_dir()
            .or_else(dirs::home_dir)
            .ok_or_else(|| StorageError::Io("Could not determine home directory".to_string()))?;

        let path = base.join("astra").join("documents");
        Self::new(path)
    }

    fn document_path(&self, id: &str) -> PathBuf {
        let safe_id: String = id
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        self.base_path.join(format!("{}.json", safe_id))
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }
}

impl Storage for FileStorage {
    fn save(&self, id: &str, document: &CanvasDocument) -> BoxFuture<'_, StorageResult<()>> {
        let path = self.document_path(id);
        let json = match document.to_json() {
            Ok(j) => j,
            Err(e) => {
                return Box::pin(async move { Err(StorageError::Serialization(e.to_string())) });
            }
        };

        Box::pin(async move {
            fs::write(&path, json)
                .map_err(|e| StorageError::Io(format!("Failed to write {}: {}", path.display(), e)))
        })
    }

    fn load(&self, id: &str) -> BoxFuture<'_, StorageResult<CanvasDocument>> {
        let path = self.document_path(id);
        let id_owned = id.to_string();

        Box::pin(async move {
            if !path.exists() {
                return Err(StorageError::NotFound(id_owned));
            }

            let json = fs::read_to_string(&path).map_err(|e| {
                StorageError::Io(format!("Failed to read {}: {}", path.display(), e))
            })?;

            CanvasDocument::from_json(&json).map_err(|e| {
                StorageError::Serialization(format!("Failed to parse {}: {}", path.display(), e))
            })
        })
    }

    fn delete(&self, id: &str) -> BoxFuture<'_, StorageResult<()>> {
        let path = self.document_path(id);

        Box::pin(async move {
            if path.exists() {
                fs::remove_file(&path).map_err(|e| {
                    StorageError::Io(format!("Failed to delete {}: {}", path.display(), e))
                })?;
            }
            Ok(())
        })
    }

    fn list(&self) -> BoxFuture<'_, StorageResult<Vec<String>>> {
        let base = self.base_path.clone();

        Box::pin(async move {
            if !base.exists() {
                return Ok(vec![]);
            }

            let entries = fs::read_dir(&base)
                .map_err(|e| StorageError::Io(format!("Failed to read directory: {}", e)))?;

            let mut ids = Vec::new();
            for entry in entries.flatten() {
                if let Some(name) = entry.path().file_stem() {
                    if let Some(name_str) = name.to_str() {
                        if entry
                            .path()
                            .extension()
                            .map(|e| e == "json")
                            .unwrap_or(false)
                        {
                            ids.push(name_str.to_string());
                        }
                    }
                }
            }
            Ok(ids)
        })
    }

    fn exists(&self, id: &str) -> BoxFuture<'_, StorageResult<bool>> {
        let path = self.document_path(id);
        Box::pin(async move { Ok(path.exists()) })
    }
}
