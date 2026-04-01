use std::collections::HashMap;
use std::fmt;

use kurbo::{Point, Rect};
use rstar::{RTree, AABB, RTreeObject, PointDistance};

use crate::shapes::{Shape, ShapeId};

#[derive(Clone, Copy, Debug, PartialEq)]
struct SpatialEntry {
    id: ShapeId,
    aabb: AABB<[f64; 2]>,
}

impl SpatialEntry {
    fn new(id: ShapeId, bounds: Rect) -> Self {
        Self {
            id,
            aabb: rect_to_aabb(bounds),
        }
    }
}

impl RTreeObject for SpatialEntry {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.aabb
    }
}

impl PointDistance for SpatialEntry {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let [px, py] = *point;
        let min = self.aabb.lower();
        let max = self.aabb.upper();
        let dx = if px < min[0] {
            min[0] - px
        } else if px > max[0] {
            px - max[0]
        } else {
            0.0
        };
        let dy = if py < min[1] {
            min[1] - py
        } else if py > max[1] {
            py - max[1]
        } else {
            0.0
        };
        dx * dx + dy * dy
    }
}

fn rect_to_aabb(r: Rect) -> AABB<[f64; 2]> {
    let x0 = r.x0.min(r.x1);
    let y0 = r.y0.min(r.y1);
    let x1 = r.x0.max(r.x1);
    let y1 = r.y0.max(r.y1);
    AABB::from_corners([x0, y0], [x1, y1])
}

pub struct UniformGrid {
    tree: RTree<SpatialEntry>,
    shape_bounds: HashMap<ShapeId, Rect>,
}

impl Clone for UniformGrid {
    fn clone(&self) -> Self {
        let shape_bounds = self.shape_bounds.clone();
        let items: Vec<SpatialEntry> = shape_bounds
            .iter()
            .map(|(&id, &bounds)| SpatialEntry::new(id, bounds))
            .collect();
        Self {
            tree: RTree::bulk_load(items),
            shape_bounds,
        }
    }
}

impl fmt::Debug for UniformGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UniformGrid")
            .field("len", &self.len())
            .finish()
    }
}

impl UniformGrid {
    pub fn new() -> Self {
        Self {
            tree: RTree::new(),
            shape_bounds: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: ShapeId, bounds: Rect) {
        if let Some(old_bounds) = self.shape_bounds.insert(id, bounds) {
            let _ = self.tree.remove(&SpatialEntry::new(id, old_bounds));
        }
        self.tree.insert(SpatialEntry::new(id, bounds));
    }

    pub fn remove(&mut self, id: ShapeId, _bounds: Rect) {
        let Some(bounds) = self.shape_bounds.remove(&id) else {
            return;
        };
        let _ = self.tree.remove(&SpatialEntry::new(id, bounds));
    }

    pub fn update(&mut self, id: ShapeId, _old_bounds: Rect, new_bounds: Rect) {
        let Some(old_stored) = self.shape_bounds.remove(&id) else {
            return;
        };
        let _ = self.tree.remove(&SpatialEntry::new(id, old_stored));
        self.shape_bounds.insert(id, new_bounds);
        self.tree.insert(SpatialEntry::new(id, new_bounds));
    }

    pub fn sync(&mut self, id: ShapeId, new_bounds: Rect) {
        match self.shape_bounds.get(&id).copied() {
            None => self.insert(id, new_bounds),
            Some(old) if old != new_bounds => self.update(id, old, new_bounds),
            Some(_) => {}
        }
    }

    pub fn query_rect(&self, viewport: Rect) -> Vec<ShapeId> {
        let envelope = rect_to_aabb(viewport);
        self.tree
            .locate_in_envelope_intersecting(&envelope)
            .map(|e| e.id)
            .collect()
    }

    pub fn query_point(&self, point: Point) -> Vec<ShapeId> {
        let p = [point.x, point.y];
        self.tree
            .locate_all_at_point(&p)
            .map(|e| e.id)
            .collect()
    }

    pub fn clear(&mut self) {
        self.tree = RTree::new();
        self.shape_bounds.clear();
    }

    pub fn rebuild(&mut self, shapes: &HashMap<ShapeId, Shape>) {
        self.shape_bounds.clear();
        let mut items = Vec::with_capacity(shapes.len());
        for (&id, shape) in shapes {
            let bounds = shape.bounds();
            self.shape_bounds.insert(id, bounds);
            items.push(SpatialEntry::new(id, bounds));
        }
        self.tree = RTree::bulk_load(items);
    }

    pub fn len(&self) -> usize {
        self.shape_bounds.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shape_bounds.is_empty()
    }
}

impl Default for UniformGrid {
    fn default() -> Self {
        Self::new()
    }
}
