use super::core::Image;
use crate::shapes::{ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{Affine, BezPath, Point, Rect, Shape as KurboShape};

impl ShapeTrait for Image {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.as_rect()
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let rect = self.as_rect().inflate(tolerance, tolerance);
        rect.contains(point)
    }

    fn to_path(&self) -> BezPath {
        self.as_rect().to_path(0.1)
    }

    fn style(&self) -> &ShapeStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.style
    }

    fn transform(&mut self, affine: Affine) {
        self.position = affine * self.position;
        let scale = affine.as_coeffs();
        self.width *= scale[0].abs();
        self.height *= scale[3].abs();
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}
