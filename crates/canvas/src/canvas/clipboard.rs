use super::state::Canvas;
use crate::shapes::{Shape, ShapeId};
use kurbo::{Affine, Rect, Vec2};

impl Canvas {
    pub fn selection_bounds(&self) -> Option<Rect> {
        self.selection
            .iter()
            .filter_map(|&id| self.document.get_shape(id).map(|shape| shape.bounds()))
            .fold(None, |acc, bounds| {
                Some(match acc {
                    Some(current) => current.union(bounds),
                    None => bounds,
                })
            })
    }

    pub fn copy_selected_as_json(&self) -> Option<String> {
        if self.selection.is_empty() {
            return None;
        }

        let shapes: Vec<Shape> = self
            .selection
            .iter()
            .filter_map(|&id| self.document.get_shape(id).cloned())
            .collect();

        serde_json::to_string(&shapes).ok()
    }

    pub fn cut_selected_as_json(&mut self) -> Option<String> {
        let json = self.copy_selected_as_json()?;
        self.document.push_undo();
        for id in self.selection.clone() {
            self.document.remove_shape(id);
        }
        self.clear_selection();
        Some(json)
    }

    pub fn paste_shapes_from_json(&mut self, json: &str, offset: Vec2) -> Option<Vec<ShapeId>> {
        let shapes = serde_json::from_str::<Vec<Shape>>(json).ok()?;
        Some(
            self.insert_shapes_and_select(shapes.into_iter().map(|mut shape| {
                shape.regenerate_id();
                shape.transform(Affine::translate(offset));
                shape
            })),
        )
    }
}
