use super::types::Group;
use crate::shapes::{Shape, ShapeId};

impl Group {
    pub fn children(&self) -> &[Shape] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut Vec<Shape> {
        &mut self.children
    }

    pub fn ungroup(self) -> Vec<Shape> {
        self.children
    }

    pub fn all_shape_ids(&self) -> Vec<ShapeId> {
        let mut ids = vec![self.id];
        for child in &self.children {
            if let Shape::Group(group) = child {
                ids.extend(group.all_shape_ids());
            } else {
                ids.push(child.id());
            }
        }
        ids
    }

    pub fn find_shape(&self, id: ShapeId) -> Option<&Shape> {
        for child in &self.children {
            if child.id() == id {
                return Some(child);
            }
            if let Shape::Group(group) = child {
                if let Some(found) = group.find_shape(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn find_shape_mut(&mut self, id: ShapeId) -> Option<&mut Shape> {
        for child in &mut self.children {
            if child.id() == id {
                return Some(child);
            }
            if let Shape::Group(group) = child {
                if let Some(found) = group.find_shape_mut(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn non_text_bounds(&self) -> Option<kurbo::Rect> {
        self.children.iter()
            .filter(|c| !matches!(c, Shape::Text(_)))
            .map(|c| c.bounds())
            .reduce(|a, b| a.union(b))
    }

    pub fn fit_and_center_text_children(&mut self) {
        let parent_bounds = match self.non_text_bounds() {
            Some(b) => b,
            None => return,
        };
        for child in &mut self.children {
            if let Shape::Text(text) = child {
                text.fit_within_bounds(parent_bounds);
            }
        }
    }
}
