use super::{ShapeId, ShapeStyle, ShapeTrait, StrokeStyle};
use kurbo::{Affine, BezPath, Point, Rect, RoundedRect, Shape as KurboShape};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rectangle {
    pub(crate) id: ShapeId,
    pub position: Point,
    pub width: f64,
    pub height: f64,
    pub corner_radius: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub stroke_style: StrokeStyle,
    pub style: ShapeStyle,
}

impl Rectangle {
    pub const DEFAULT_ADAPTIVE_RADIUS: f64 = 32.0;

    pub const DEFAULT_PROPORTIONAL_RADIUS: f64 = 0.25;

    pub fn new(position: Point, width: f64, height: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            width,
            height,
            corner_radius: 0.0,
            rotation: 0.0,
            stroke_style: StrokeStyle::default(),
            style: ShapeStyle::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: ShapeId,
        position: Point,
        width: f64,
        height: f64,
        corner_radius: f64,
        rotation: f64,
        stroke_style: StrokeStyle,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            position,
            width,
            height,
            corner_radius,
            rotation,
            stroke_style,
            style,
        }
    }

    pub fn from_corners(p1: Point, p2: Point) -> Self {
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);
        let width = (p2.x - p1.x).abs();
        let height = (p2.y - p1.y).abs();

        Self::new(Point::new(min_x, min_y), width, height)
    }

    pub fn as_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.width,
            self.position.y + self.height,
        )
    }
}

impl ShapeTrait for Rectangle {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        self.as_rect()
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        self.as_rect().inflate(tolerance, tolerance).contains(point)
    }

    fn to_path(&self) -> BezPath {
        if self.corner_radius > 0.0 {
            let rounded = RoundedRect::from_rect(self.as_rect(), self.corner_radius);
            rounded.to_path(0.1)
        } else {
            self.as_rect().to_path(0.1)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_creation() {
        let rect = Rectangle::new(Point::new(10.0, 20.0), 100.0, 50.0);
        assert!((rect.position.x - 10.0).abs() < f64::EPSILON);
        assert!((rect.position.y - 20.0).abs() < f64::EPSILON);
        assert!((rect.width - 100.0).abs() < f64::EPSILON);
        assert!((rect.height - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_rectangle_from_corners() {
        let rect = Rectangle::from_corners(Point::new(100.0, 100.0), Point::new(50.0, 50.0));
        assert!((rect.position.x - 50.0).abs() < f64::EPSILON);
        assert!((rect.position.y - 50.0).abs() < f64::EPSILON);
        assert!((rect.width - 50.0).abs() < f64::EPSILON);
        assert!((rect.height - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hit_test() {
        let rect = Rectangle::new(Point::new(0.0, 0.0), 100.0, 100.0);
        assert!(rect.hit_test(Point::new(50.0, 50.0), 0.0));
        assert!(!rect.hit_test(Point::new(150.0, 50.0), 0.0));
        assert!(rect.hit_test(Point::new(105.0, 50.0), 10.0));
    }

    #[test]
    fn test_bounds() {
        let rect = Rectangle::new(Point::new(10.0, 20.0), 100.0, 50.0);
        let bounds = rect.bounds();
        assert!((bounds.x0 - 10.0).abs() < f64::EPSILON);
        assert!((bounds.y0 - 20.0).abs() < f64::EPSILON);
        assert!((bounds.x1 - 110.0).abs() < f64::EPSILON);
        assert!((bounds.y1 - 70.0).abs() < f64::EPSILON);
    }
}
