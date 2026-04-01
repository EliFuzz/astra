use crate::vello_impl::VelloRenderer;
use kurbo::{Affine, BezPath, Point};
use peniko::Fill;

impl VelloRenderer {
    pub(crate) fn render_freehand_with_pressure(
        &mut self,
        freehand: &astra_canvas::shapes::Freehand,
        transform: Affine,
    ) {
        use kurbo::Vec2;

        if freehand.points.len() < 2 {
            return;
        }

        let style = &freehand.style;
        let base_size = style.stroke_width * 2.3;
        let thinning = 0.6;
        let color = style.stroke_with_opacity();

        let mut left_points: Vec<Point> = Vec::new();
        let mut right_points: Vec<Point> = Vec::new();

        for i in 0..freehand.points.len() {
            let point = freehand.points[i];
            let pressure = freehand.pressure_at(i);
            let eased_pressure = (pressure * std::f64::consts::PI / 2.0).sin();
            let width = base_size * (1.0 - thinning * (1.0 - eased_pressure));

            let dir = if i == 0 {
                let next = freehand.points[i + 1];
                Vec2::new(next.x - point.x, next.y - point.y)
            } else if i == freehand.points.len() - 1 {
                let prev = freehand.points[i - 1];
                Vec2::new(point.x - prev.x, point.y - prev.y)
            } else {
                let prev = freehand.points[i - 1];
                let next = freehand.points[i + 1];
                Vec2::new(next.x - prev.x, next.y - prev.y)
            };

            let len = dir.hypot();
            if len < f64::EPSILON {
                continue;
            }

            let perp = Vec2::new(-dir.y / len, dir.x / len);
            let half_width = width / 2.0;

            left_points.push(Point::new(
                point.x + perp.x * half_width,
                point.y + perp.y * half_width,
            ));
            right_points.push(Point::new(
                point.x - perp.x * half_width,
                point.y - perp.y * half_width,
            ));
        }

        if left_points.is_empty() {
            return;
        }

        let mut path = BezPath::new();
        path.move_to(left_points[0]);
        for point in left_points.iter().skip(1) {
            path.line_to(*point);
        }
        for point in right_points.iter().rev() {
            path.line_to(*point);
        }
        path.close_path();

        self.scene
            .fill(Fill::NonZero, transform, color, None, &path);
    }
}
