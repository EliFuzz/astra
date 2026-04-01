use super::Text;
use crate::shapes::{ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{Affine, BezPath, Point, Rect};

impl ShapeTrait for Text {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        let (width, height) = self
            .cached_size
            .read()
            .ok()
            .and_then(|guard| *guard)
            .map(|(w, h)| (w.max(20.0), h))
            .unwrap_or_else(|| {
                (
                    self.approximate_width().max(20.0),
                    self.approximate_height(),
                )
            });
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + width,
            self.position.y + height,
        )
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let bounds = self.bounds().inflate(tolerance, tolerance);
        bounds.contains(point)
    }

    fn to_path(&self) -> BezPath {
        let bounds = self.bounds();
        let mut path = BezPath::new();
        path.move_to(Point::new(bounds.x0, bounds.y0));
        path.line_to(Point::new(bounds.x1, bounds.y0));
        path.line_to(Point::new(bounds.x1, bounds.y1));
        path.line_to(Point::new(bounds.x0, bounds.y1));
        path.close_path();
        path
    }

    fn style(&self) -> &ShapeStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.style
    }

    fn transform(&mut self, affine: Affine) {
        self.position = affine * self.position;
        let coeffs = affine.as_coeffs();
        let scale = (coeffs[0].abs() + coeffs[3].abs()) / 2.0;
        if (scale - 1.0).abs() > 0.01 {
            self.font_size *= scale;
        }
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::Text;
    use crate::shapes::ShapeTrait;
    use kurbo::Point;

    #[test]
    fn test_text_creation() {
        let text = Text::new(Point::new(100.0, 100.0), "Hello".to_string());
        assert_eq!(text.content(), "Hello");
        assert!((text.font_size - Text::DEFAULT_FONT_SIZE).abs() < f64::EPSILON);
    }

    #[test]
    fn test_text_with_font_size() {
        let text = Text::new(Point::new(0.0, 0.0), "Test".to_string()).with_font_size(32.0);
        assert!((text.font_size - 32.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_hit_test() {
        let text = Text::new(Point::new(100.0, 100.0), "Hello World".to_string());
        let bounds = text.bounds();
        let center = Point::new((bounds.x0 + bounds.x1) / 2.0, (bounds.y0 + bounds.y1) / 2.0);
        assert!(text.hit_test(center, 0.0));
        assert!(!text.hit_test(Point::new(0.0, 0.0), 0.0));
    }

    #[test]
    fn test_bounds() {
        let text = Text::new(Point::new(100.0, 100.0), "Hi".to_string());
        let bounds = text.bounds();
        assert!(bounds.width() > 0.0);
        assert!(bounds.height() > 0.0);
    }
}
