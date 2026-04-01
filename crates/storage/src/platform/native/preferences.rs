use crate::preferences::UserPreferences;
use crate::{StorageError, StorageResult};

pub fn save_to_file(prefs: &UserPreferences) -> StorageResult<()> {
    let base = dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| StorageError::Io("Could not determine home directory".to_string()))?;

    let dir = base.join("astra");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| StorageError::Io(format!("Failed to create directory: {}", e)))?;
    }

    let json = prefs
        .to_json()
        .map_err(|e| StorageError::Serialization(e.to_string()))?;

    std::fs::write(dir.join("preferences.json"), json)
        .map_err(|e| StorageError::Io(format!("Failed to write preferences: {}", e)))
}

pub fn load_from_file() -> StorageResult<UserPreferences> {
    let base = dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| StorageError::Io("Could not determine home directory".to_string()))?;

    let path = base.join("astra").join("preferences.json");
    if !path.exists() {
        return Ok(UserPreferences::default());
    }

    let json = std::fs::read_to_string(&path)
        .map_err(|e| StorageError::Io(format!("Failed to read preferences: {}", e)))?;

    UserPreferences::from_json(&json)
        .map_err(|e| StorageError::Serialization(format!("Failed to parse preferences: {}", e)))
}
