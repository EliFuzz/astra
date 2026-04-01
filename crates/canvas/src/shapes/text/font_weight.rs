use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FontWeight {
    Light,
    #[default]
    Regular,
    Heavy,
}

impl FontWeight {
    pub fn display_name(&self) -> &'static str {
        match self {
            FontWeight::Light => "Light",
            FontWeight::Regular => "Regular",
            FontWeight::Heavy => "Heavy",
        }
    }

    pub fn all() -> &'static [FontWeight] {
        &[FontWeight::Light, FontWeight::Regular, FontWeight::Heavy]
    }
}
