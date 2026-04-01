use kurbo::{Affine, BezPath, Point, Rect};
use uuid::Uuid;

use crate::style::ShapeStyle;

pub type ShapeId = Uuid;

pub trait ShapeTrait {
    fn id(&self) -> ShapeId;

    fn bounds(&self) -> Rect;

    fn hit_test(&self, point: Point, tolerance: f64) -> bool;

    fn to_path(&self) -> BezPath;

    fn style(&self) -> &ShapeStyle;

    fn style_mut(&mut self) -> &mut ShapeStyle;

    fn transform(&mut self, affine: Affine);

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync>;
}
