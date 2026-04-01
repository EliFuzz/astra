use super::super::{Shape, ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{Affine, BezPath, Point, Rect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub(crate) id: ShapeId,
    pub children: Vec<Shape>,
    #[serde(default)]
    pub rotation: f64,
    style: ShapeStyle,
}

impl Group {
    pub fn new(children: Vec<Shape>) -> Self {
        Self {
            id: Uuid::new_v4(),
            children,
            rotation: 0.0,
            style: ShapeStyle::default(),
        }
    }

    pub fn reconstruct(id: ShapeId, children: Vec<Shape>) -> Self {
        Self {
            id,
            children,
            rotation: 0.0,
            style: ShapeStyle::default(),
        }
    }
}

impl ShapeTrait for Group {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        if self.children.is_empty() {
            return Rect::ZERO;
        }

        let mut bounds = self.children[0].bounds();
        for child in &self.children[1..] {
            bounds = bounds.union(child.bounds());
        }
        bounds
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        for child in &self.children {
            if child.hit_test(point, tolerance) {
                return true;
            }
        }
        false
    }

    fn to_path(&self) -> BezPath {
        let mut path = BezPath::new();
        for child in &self.children {
            path.extend(child.to_path());
        }
        path
    }

    fn style(&self) -> &ShapeStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.style
    }

    fn transform(&mut self, affine: Affine) {
        for child in &mut self.children {
            child.transform(affine);
        }
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}
