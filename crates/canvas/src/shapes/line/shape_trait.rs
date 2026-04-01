use super::{Line, PathStyle};
use crate::shapes::{ShapeId, ShapeStyle, ShapeTrait, point_to_polyline_dist};
use kurbo::{Affine, BezPath, Point, Rect};

impl ShapeTrait for Line {
    fn id(&self) -> ShapeId {
        self.id
    }

    fn bounds(&self) -> Rect {
        let points = self.all_points();
        let (min_x, max_x) = points.iter().fold((f64::MAX, f64::MIN), |(mn, mx), p| {
            (mn.min(p.x), mx.max(p.x))
        });
        let (min_y, max_y) = points.iter().fold((f64::MAX, f64::MIN), |(mn, mx), p| {
            (mn.min(p.y), mx.max(p.y))
        });
        Rect::new(min_x, min_y, max_x, max_y)
    }

    fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let points = self.all_points();
        if points.len() < 2 {
            return false;
        }
        let dist = point_to_polyline_dist(point, &points);
        dist <= tolerance + self.style.stroke_width / 2.0
    }

    fn to_path(&self) -> BezPath {
        let mut path = BezPath::new();

        if self.start == self.end {
            return path;
        }

        let points = match self.path_style {
            PathStyle::Angular if self.intermediate_points.is_empty() => {
                let elbow_pts = crate::elbow::compute_elbow_path(self.start, None, self.end, None);
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
        self.start = affine * self.start;
        self.end = affine * self.end;
        for p in &mut self.intermediate_points {
            *p = affine * *p;
        }
    }

    fn clone_box(&self) -> Box<dyn ShapeTrait + Send + Sync> {
        Box::new(self.clone())
    }
}
