use super::platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FontFamily {
    #[default]
    Handwriting,
    #[serde(alias = "NotoSans")]
    Sans,
}

impl FontFamily {
    pub fn system_font_stack(&self) -> &'static str {
        match self {
            FontFamily::Handwriting => platform::handwriting_font_stack(),
            FontFamily::Sans => platform::sans_font_stack(),
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            FontFamily::Handwriting => "Handwriting",
            FontFamily::Sans => "Sans",
        }
    }

    pub fn all() -> &'static [FontFamily] {
        &[FontFamily::Handwriting, FontFamily::Sans]
    }
}
