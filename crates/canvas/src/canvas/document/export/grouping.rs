use super::super::CanvasDocument;
use crate::shapes::{Group, Shape, ShapeId, ShapeTrait};

impl CanvasDocument {
    pub fn export_selection(&self, selection: &[ShapeId]) -> Self {
        let mut doc = Self::new();
        for &id in selection {
            if let Some(shape) = self.shapes.get(&id) {
                doc.add_shape(shape.clone());
            }
        }
        doc
    }

    pub fn group_shapes(&mut self, shape_ids: &[ShapeId]) -> Option<ShapeId> {
        if shape_ids.len() < 2 {
            return None;
        }

        let mut shapes_to_group: Vec<(usize, Shape)> = Vec::new();
        for (idx, &zid) in self.z_order.iter().enumerate() {
            if shape_ids.contains(&zid) {
                if let Some(shape) = self.shapes.get(&zid) {
                    shapes_to_group.push((idx, shape.clone()));
                }
            }
        }

        if shapes_to_group.len() < 2 {
            return None;
        }

        let max_z_idx = shapes_to_group.iter().map(|(idx, _)| *idx).max().unwrap();
        let children: Vec<Shape> = shapes_to_group.into_iter().map(|(_, s)| s).collect();
        let group = Group::new(children);
        let group_id = group.id();

        for &id in shape_ids {
            let bounds = self.shapes.get(&id).map(|s| s.bounds());
            self.shapes.remove(&id);
            self.z_order.retain(|&zid| zid != id);
            if let Some(b) = bounds {
                self.spatial_index.remove(id, b);
            }
        }

        let group_shape = Shape::Group(group);
        let group_bounds = group_shape.bounds();
        self.shapes.insert(group_id, group_shape);
        self.spatial_index.insert(group_id, group_bounds);
        let insert_pos = max_z_idx
            .saturating_sub(shape_ids.len() - 1)
            .min(self.z_order.len());
        self.z_order.insert(insert_pos, group_id);

        Some(group_id)
    }

    pub fn ungroup_shape(&mut self, group_id: ShapeId) -> Option<Vec<ShapeId>> {
        let group = match self.shapes.get(&group_id) {
            Some(Shape::Group(g)) => g.clone(),
            _ => return None,
        };

        let z_pos = self.z_order.iter().position(|&id| id == group_id)?;

        let g_bounds = self.shapes.get(&group_id).map(|s| s.bounds());
        self.shapes.remove(&group_id);
        self.z_order.retain(|&id| id != group_id);
        if let Some(b) = g_bounds {
            self.spatial_index.remove(group_id, b);
        }

        let children = group.ungroup();
        let child_ids: Vec<ShapeId> = children.iter().map(|s| s.id()).collect();

        for (i, child) in children.into_iter().enumerate() {
            let child_id = child.id();
            let cb = child.bounds();
            self.shapes.insert(child_id, child);
            self.spatial_index.insert(child_id, cb);
            self.z_order.insert(z_pos + i, child_id);
        }

        Some(child_ids)
    }
}
