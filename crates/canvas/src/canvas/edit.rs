use super::state::Canvas;
use crate::shapes::{Shape, ShapeId};

impl Canvas {
    pub fn update_math_latex(&mut self, shape_id: ShapeId, latex: &str) -> bool {
        self.document.push_undo();
        if let Some(mut shape) = self.document.get_shape_mut(shape_id) {
            if let Shape::Math(math) = &mut *shape {
                math.set_latex(latex.to_string());
                return true;
            }
        }
        false
    }

    pub fn delete_selected(&mut self) {
        for id in self.selection.drain(..).collect::<Vec<_>>() {
            self.document.remove_shape(id);
            self.widgets.remove(id);
        }
    }

    pub fn flip_selected_horizontal(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        let mut combined_bounds: Option<kurbo::Rect> = None;
        for &id in &self.selection {
            if let Some(shape) = self.document.get_shape(id) {
                let bounds = shape.bounds();
                combined_bounds = Some(match combined_bounds {
                    Some(cb) => cb.union(bounds),
                    None => bounds,
                });
            }
        }

        let Some(bounds) = combined_bounds else {
            return;
        };
        let center_x = bounds.center().x;

        for &id in &self.selection {
            if let Some(mut shape) = self.document.get_shape_mut(id) {
                let flip = kurbo::Affine::translate(kurbo::Vec2::new(center_x, 0.0))
                    * kurbo::Affine::scale_non_uniform(-1.0, 1.0)
                    * kurbo::Affine::translate(kurbo::Vec2::new(-center_x, 0.0));
                shape.transform(flip);
            }
        }
    }

    pub fn flip_selected_vertical(&mut self) {
        if self.selection.is_empty() {
            return;
        }

        let mut combined_bounds: Option<kurbo::Rect> = None;
        for &id in &self.selection {
            if let Some(shape) = self.document.get_shape(id) {
                let bounds = shape.bounds();
                combined_bounds = Some(match combined_bounds {
                    Some(cb) => cb.union(bounds),
                    None => bounds,
                });
            }
        }

        let Some(bounds) = combined_bounds else {
            return;
        };
        let center_y = bounds.center().y;

        for &id in &self.selection {
            if let Some(mut shape) = self.document.get_shape_mut(id) {
                let flip = kurbo::Affine::translate(kurbo::Vec2::new(0.0, center_y))
                    * kurbo::Affine::scale_non_uniform(1.0, -1.0)
                    * kurbo::Affine::translate(kurbo::Vec2::new(0.0, -center_y));
                shape.transform(flip);
            }
        }
    }

    pub fn remove_shape(&mut self, id: ShapeId) {
        self.selection.retain(|&s| s != id);
        self.document.remove_shape(id);
        self.widgets.remove(id);
    }

    pub fn group_selected(&mut self) -> Option<ShapeId> {
        if self.selection.len() < 2 {
            return None;
        }

        let shape_ids: Vec<ShapeId> = self.selection.clone();

        self.document.push_undo();

        if let Some(group_id) = self.document.group_shapes(&shape_ids) {
            for &id in &shape_ids {
                self.widgets.remove(id);
            }

            self.selection.clear();
            self.selection.push(group_id);
            self.widgets.add_to_selection(group_id);

            Some(group_id)
        } else {
            None
        }
    }

    pub fn ungroup_selected(&mut self) -> Vec<ShapeId> {
        let mut all_children = Vec::new();

        let groups: Vec<ShapeId> = self
            .selection
            .iter()
            .filter(|&&id| {
                self.document
                    .get_shape(id)
                    .map(|s| s.is_group())
                    .unwrap_or(false)
            })
            .copied()
            .collect();

        if groups.is_empty() {
            return all_children;
        }

        self.document.push_undo();

        for group_id in groups {
            self.widgets.remove(group_id);
            self.selection.retain(|&id| id != group_id);

            if let Some(children) = self.document.ungroup_shape(group_id) {
                for &child_id in &children {
                    if !self.selection.contains(&child_id) {
                        self.selection.push(child_id);
                    }
                    self.widgets.add_to_selection(child_id);
                }
                all_children.extend(children);
            }
        }

        all_children
    }
}
