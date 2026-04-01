use crate::shapes::{Shape, ShapeId};

use super::types::CanvasDocument;

impl CanvasDocument {
    pub fn add_shape(&mut self, shape: Shape) {
        let id = shape.id();
        let bounds = shape.bounds();
        self.z_order.push(id);
        self.shapes.insert(id, shape);
        self.spatial_index.insert(id, bounds);
    }

    pub fn remove_shape(&mut self, id: ShapeId) -> Option<Shape> {
        let bounds = self.shapes.get(&id).map(|s| s.bounds());
        self.z_order.retain(|&shape_id| shape_id != id);
        let shape = self.shapes.remove(&id)?;
        if let Some(b) = bounds {
            self.spatial_index.remove(id, b);
        }
        Some(shape)
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
        self.z_order.clear();
        self.spatial_index.clear();
    }

    pub fn get_shape(&self, id: ShapeId) -> Option<&Shape> {
        self.shapes.get(&id)
    }

    pub fn get_shape_deep(&self, id: ShapeId) -> Option<&Shape> {
        if let Some(shape) = self.shapes.get(&id) {
            return Some(shape);
        }
        for shape in self.shapes.values() {
            if let Shape::Group(g) = shape {
                if let Some(found) = g.find_shape(id) {
                    return Some(found);
                }
            }
        }
        None
    }

    pub fn get_shape_mut(&mut self, id: ShapeId) -> Option<super::ShapeMut<'_>> {
        super::ShapeMut::new(self, id)
    }

    pub fn with_group_child_mut<F, R>(&mut self, child_id: ShapeId, f: F) -> Option<R>
    where
        F: FnOnce(&mut Shape) -> R,
    {
        let group_id = self.shapes.iter().find_map(|(&gid, shape)| {
            if let Shape::Group(g) = shape {
                if g.children.iter().any(|c| c.id() == child_id) {
                    return Some(gid);
                }
            }
            None
        })?;
        if let Some(Shape::Group(g)) = self.shapes.get_mut(&group_id) {
            if let Some(child) = g.find_shape_mut(child_id) {
                return Some(f(child));
            }
        }
        None
    }

    pub fn shapes_ordered(&self) -> impl Iterator<Item = &Shape> {
        self.z_order.iter().filter_map(|id| self.shapes.get(id))
    }

    pub fn bring_to_front(&mut self, id: ShapeId) {
        self.z_order.retain(|&shape_id| shape_id != id);
        self.z_order.push(id);
    }

    pub fn send_to_back(&mut self, id: ShapeId) {
        self.z_order.retain(|&shape_id| shape_id != id);
        self.z_order.insert(0, id);
    }

    pub fn bring_forward(&mut self, id: ShapeId) -> bool {
        let Some(pos) = self.z_order.iter().position(|&shape_id| shape_id == id) else {
            return false;
        };
        if pos >= self.z_order.len() - 1 {
            return false;
        }
        self.z_order.swap(pos, pos + 1);
        true
    }

    pub fn send_backward(&mut self, id: ShapeId) -> bool {
        let Some(pos) = self.z_order.iter().position(|&shape_id| shape_id == id) else {
            return false;
        };
        if pos == 0 {
            return false;
        }
        self.z_order.swap(pos, pos - 1);
        true
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }
}
