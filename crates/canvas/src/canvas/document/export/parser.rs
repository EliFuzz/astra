use super::super::CanvasDocument;
use super::{elements, shape_style};
use crate::shapes::Shape;

impl CanvasDocument {
    pub fn shapes_from_json(json: &str) -> Result<Self, String> {
        let data: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("Invalid JSON: {}", e))?;

        let elements_arr = data
            .get("elements")
            .and_then(|e| e.as_array())
            .ok_or("Missing 'elements' array")?;

        let mut doc = Self::new();

        for elem in elements_arr {
            if elem
                .get("isDeleted")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                continue;
            }

            let elem_type = elem.get("type").and_then(|t| t.as_str()).unwrap_or("");
            let x = elem.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let y = elem.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);

            let shape_style = shape_style::shape_style_from_element(elem);
            if let Some(s) =
                elements::shape_from_element(elem, elem_type, x, y, shape_style)
            {
                doc.add_shape(s);
            }
        }

        Ok(doc)
    }

    pub fn shapes_from_clipboard(text: &str) -> Option<Vec<Shape>> {
        let data: serde_json::Value = serde_json::from_str(text).ok()?;
        let clip_type = data.get("type").and_then(|t| t.as_str())?;
        if clip_type != "excalidraw/clipboard" {
            return None;
        }
        let doc = Self::shapes_from_json(text).ok()?;
        let shapes: Vec<Shape> = doc.shapes_ordered().cloned().collect();
        if shapes.is_empty() {
            None
        } else {
            Some(shapes)
        }
    }
}
