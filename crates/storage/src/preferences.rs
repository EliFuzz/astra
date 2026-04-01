use astra_core::preferences::PreferredGridStyle;
use astra_core::SerializableColor;
use serde::{Deserialize, Serialize};

pub const PREFERENCES_KEY: &str = "__preferences__";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub grid_style: PreferredGridStyle,
    pub background_color: SerializableColor,
    pub dark_theme: bool,
    pub grid_snap: bool,
    pub smart_guides: bool,
    pub angle_snap: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            grid_style: PreferredGridStyle::None,
            background_color: SerializableColor::new(18, 18, 24, 255),
            dark_theme: true,
            grid_snap: false,
            smart_guides: false,
            angle_snap: false,
        }
    }
}

impl UserPreferences {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::native::preferences::{load_from_file, save_to_file};
