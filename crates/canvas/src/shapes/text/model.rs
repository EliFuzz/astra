mod ops;

use crate::shapes::{SerializableColor, ShapeId, ShapeStyle};
use kurbo::Point;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

use super::font_family::FontFamily;
use super::font_weight::FontWeight;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub(crate) id: ShapeId,
    pub position: Point,
    pub content: String,
    pub font_size: f64,
    pub font_family: FontFamily,
    pub font_weight: FontWeight,
    #[serde(default)]
    pub rotation: f64,
    pub style: ShapeStyle,
    #[serde(default)]
    pub char_colors: Vec<Option<SerializableColor>>,
    #[serde(default)]
    pub text_align: u8,
    #[serde(skip)]
    pub(crate) cached_size: RwLock<Option<(f64, f64)>>,
}

impl Clone for Text {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            position: self.position,
            content: self.content.clone(),
            font_size: self.font_size,
            font_family: self.font_family,
            font_weight: self.font_weight,
            rotation: self.rotation,
            style: self.style.clone(),
            char_colors: self.char_colors.clone(),
            text_align: self.text_align,
            cached_size: RwLock::new(self.cached_size.read().ok().and_then(|guard| *guard)),
        }
    }
}
