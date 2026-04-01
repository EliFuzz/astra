use crate::shapes::{Shape, ShapeId};
use kurbo::{Point, Rect};
use std::collections::HashSet;

use super::types::CanvasDocument;

impl CanvasDocument {
    pub fn bounds(&self) -> Option<Rect> {
        let mut result: Option<Rect> = None;
        for shape in self.shapes.values() {
            let bounds = shape.bounds();
            result = Some(match result {
                Some(r) => r.union(bounds),
                None => bounds,
            });
        }
        result
    }

    pub fn shapes_at_point(&self, point: Point, tolerance: f64) -> Vec<ShapeId> {
        self.z_order
            .iter()
            .rev()
            .filter_map(|&id| {
                self.shapes
                    .get(&id)
                    .filter(|s| s.hit_test(point, tolerance))
                    .map(|_| id)
            })
            .collect()
    }

    pub fn shapes_in_rect(&self, rect: Rect) -> Vec<ShapeId> {
        self.z_order
            .iter()
            .filter_map(|&id| {
                self.shapes
                    .get(&id)
                    .filter(|s| s.intersects_rect(rect))
                    .map(|_| id)
            })
            .collect()
    }

    pub fn visible_shapes_ordered(&self, viewport: Rect) -> Vec<&Shape> {
        let candidates: HashSet<ShapeId> =
            self.spatial_index.query_rect(viewport).into_iter().collect();
        self.z_order
            .iter()
            .filter_map(|id| {
                if candidates.contains(id) {
                    self.shapes.get(id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn arrows_bound_to(&self, shape_id: ShapeId) -> Vec<ShapeId> {
        self.z_order
            .iter()
            .filter_map(|&id| self.shapes.get(&id))
            .filter_map(|s| {
                let Shape::Arrow(a) = s else {
                    return None;
                };
                let start_bound = a
                    .start_binding
                    .as_ref()
                    .map(|b| b.target_id == shape_id)
                    .unwrap_or(false);
                let end_bound = a
                    .end_binding
                    .as_ref()
                    .map(|b| b.target_id == shape_id)
                    .unwrap_or(false);
                if !start_bound && !end_bound {
                    return None;
                }
                Some(a.id)
            })
            .collect()
    }
}
