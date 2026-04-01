use super::shape::Arrow;
use super::super::line::PathStyle;
use super::super::{ShapeId, ShapeStyle, ShapeTrait};
use kurbo::{Affine, BezPath, Point, Rect, Vec2};

impl ShapeTrait for Arrow {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        let dir = self.direction();
        let perp = Vec2::new(-dir.y, dir.x);

        let head_back = Point::new(
            self.end.x - dir.x * self.head_size,
            self.end.y - dir.y * self.head_size,
        );
        let head_left = Point::new(
            head_back.x + perp.x * self.head_size * 0.5,
            head_back.y + perp.y * self.head_size * 0.5,
        );
        let head_right = Point::new(
            head_back.x - perp.x * self.head_size * 0.5,
            head_back.y - perp.y * self.head_size * 0.5,
        );

        let points = self.all_points();
        let mut min_x = head_left.x.min(head_right.x);
        let mut min_y = head_left.y.min(head_right.y);
        let mut max_x = head_left.x.max(head_right.x);
        let mut max_y = head_left.y.max(head_right.y);
        for p in &points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        Rect::new(min_x, min_y, max_x, max_y)
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let points = self.all_points();
        if points.len() >= 2 {
            let dist = super::super::point_to_polyline_dist(point, &points);
            if dist <= tolerance + self.style.stroke_width / 2.0 {
                return true;
            }
        }

        let dir = self.direction();
        let perp = Vec2::new(-dir.y, dir.x);
        let head_back = Point::new(
            self.end.x - dir.x * self.head_size,
            self.end.y - dir.y * self.head_size,
        );
        let head_left = Point::new(
            head_back.x + perp.x * self.head_size * 0.5,
            head_back.y + perp.y * self.head_size * 0.5,
        );
        let head_right = Point::new(
            head_back.x - perp.x * self.head_size * 0.5,
            head_back.y - perp.y * self.head_size * 0.5,
        );

        fn sign(p1: Point, p2: Point, p3: Point) -> f64 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        }

        let d1 = sign(point, self.end, head_left);
        let d2 = sign(point, head_left, head_right);
        let d3 = sign(point, head_right, self.end);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        !(has_neg && has_pos)
    }

    fn to_path(&self) -> BezPath {
        let mut path = BezPath::new();

        if self.start == self.end {
            return path;
        }

        let points = match self.path_style {
            PathStyle::Angular if self.intermediate_points.is_empty() => {
                let exit = self.start_binding.as_ref().map(|b| b.side);
                let entry = self.end_binding.as_ref().map(|b| b.side);
                let elbow_pts = crate::elbow::compute_elbow_path(self.start, exit, self.end, entry);
                let mut pts = vec![self.start];
                pts.extend(elbow_pts);
                pts.push(self.end);
                pts
            }
            _ => self.all_points(),
        };

        if points.len() < 2 {
            return path;
        }

        path.move_to(points[0]);

        match self.path_style {
            PathStyle::Direct | PathStyle::Angular => {
                for p in &points[1..] {
                    path.line_to(*p);
                }
            }
            PathStyle::Flowing => {
                let tension = 0.5;
                for i in 0..points.len() - 1 {
                    let p0 = points[if i == 0 { 0 } else { i - 1 }];
                    let p1 = points[i];
                    let p2 = points[i + 1];
                    let p3 = points[if i + 2 >= points.len() {
                        points.len() - 1
                    } else {
                        i + 2
                    }];

                    let t1x = (p2.x - p0.x) * tension;
                    let t1y = (p2.y - p0.y) * tension;
                    let t2x = (p3.x - p1.x) * tension;
                    let t2y = (p3.y - p1.y) * tension;

                    let cp1 = Point::new(p1.x + t1x / 3.0, p1.y + t1y / 3.0);
                    let cp2 = Point::new(p2.x - t2x / 3.0, p2.y - t2y / 3.0);

                    path.curve_to(cp1, cp2, p2);
                }
            }
        }

        let last_pt = points[points.len() - 1];
        let prev_pt = points[points.len() - 2];
        let dx = last_pt.x - prev_pt.x;
        let dy = last_pt.y - prev_pt.y;
        let len = (dx * dx + dy * dy).sqrt();
        let dir = if len > f64::EPSILON {
            Vec2::new(dx / len, dy / len)
        } else {
            self.direction()
        };
        let perp = Vec2::new(-dir.y, dir.x);

        let head_back = Point::new(
            self.end.x - dir.x * self.head_size,
            self.end.y - dir.y * self.head_size,
        );
        let head_left = Point::new(
            head_back.x + perp.x * self.head_size * 0.5,
            head_back.y + perp.y * self.head_size * 0.5,
        );
        let head_right = Point::new(
            head_back.x - perp.x * self.head_size * 0.5,
            head_back.y - perp.y * self.head_size * 0.5,
        );

        path.move_to(self.end);
        path.line_to(head_left);
        path.move_to(self.end);
        path.line_to(head_right);

        path
    }

    fn style(&self) -> &ShapeStyle {
        &self.style
    }

    fn style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.style
    }

    fn transform(&mut self, affine: Affine) {
        self.start = affine * self.start;
        self.end = affine * self.end;
        for p in &mut self.intermediate_points {
            *p = affine * *p;
        }
        let scale = affine.as_coeffs();
        self.head_size *= (scale[0].abs() + scale[3].abs()) / 2.0;
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}
