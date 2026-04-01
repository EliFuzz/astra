use super::data::Freehand;
use crate::shapes::{ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{Affine, BezPath, Point, Rect};

impl ShapeTrait for Freehand {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        if self.points.is_empty() {
            return Rect::ZERO;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in &self.points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Rect::new(min_x, min_y, max_x, max_y)
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        if self.points.len() < 2 {
            if let Some(p) = self.points.first() {
                let dx = point.x - p.x;
                let dy = point.y - p.y;
                return (dx * dx + dy * dy).sqrt() <= tolerance;
            }
            return false;
        }

        for window in self.points.windows(2) {
            let start = window[0];
            let end = window[1];

            let line_vec = kurbo::Vec2::new(end.x - start.x, end.y - start.y);
            let point_vec = kurbo::Vec2::new(point.x - start.x, point.y - start.y);

            let line_len_sq = line_vec.hypot2();
            if line_len_sq < f64::EPSILON {
                continue;
            }

            let t = (point_vec.dot(line_vec) / line_len_sq).clamp(0.0, 1.0);
            let projection = Point::new(start.x + t * line_vec.x, start.y + t * line_vec.y);

            let dist = ((point.x - projection.x).powi(2) + (point.y - projection.y).powi(2)).sqrt();
            if dist <= tolerance + self.style.stroke_width / 2.0 {
                return true;
            }
        }

        false
    }

    fn to_path(&self) -> BezPath {
        let mut path = BezPath::new();

        if self.points.is_empty() {
            return path;
        }

        path.move_to(self.points[0]);
        for point in self.points.iter().skip(1) {
            path.line_to(*point);
        }
        if self.closed {
            path.close_path();
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
        for point in &mut self.points {
            *point = affine * *point;
        }
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}
