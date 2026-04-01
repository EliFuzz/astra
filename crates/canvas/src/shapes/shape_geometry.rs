use super::arrow::BindSide;
use kurbo::{Point, Rect};

pub fn line_segments_intersect_rect(points: &[Point], rect: Rect) -> bool {
    if points.iter().any(|p| rect.contains(*p)) {
        return true;
    }
    let corners = [
        Point::new(rect.x0, rect.y0),
        Point::new(rect.x1, rect.y0),
        Point::new(rect.x1, rect.y1),
        Point::new(rect.x0, rect.y1),
    ];
    let edges = [
        (corners[0], corners[1]),
        (corners[1], corners[2]),
        (corners[2], corners[3]),
        (corners[3], corners[0]),
    ];
    for w in points.windows(2) {
        let (a, b) = (w[0], w[1]);
        for &(c, d) in &edges {
            if segments_intersect(a, b, c, d) {
                return true;
            }
        }
    }
    false
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let cross = |o: Point, p: Point, q: Point| -> f64 {
        (p.x - o.x) * (q.y - o.y) - (p.y - o.y) * (q.x - o.x)
    };
    let d1 = cross(c, d, a);
    let d2 = cross(c, d, b);
    let d3 = cross(a, b, c);
    let d4 = cross(a, b, d);
    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }
    let on_segment = |p: Point, q: Point, r: Point| -> bool {
        r.x >= p.x.min(q.x) && r.x <= p.x.max(q.x) && r.y >= p.y.min(q.y) && r.y <= p.y.max(q.y)
    };
    (d1.abs() < 1e-10 && on_segment(c, d, a))
        || (d2.abs() < 1e-10 && on_segment(c, d, b))
        || (d3.abs() < 1e-10 && on_segment(a, b, c))
        || (d4.abs() < 1e-10 && on_segment(a, b, d))
}

pub(crate) fn closest_point_on_rect_border(p: Point, r: Rect) -> Point {
    let cx = p.x.clamp(r.x0, r.x1);
    let cy = p.y.clamp(r.y0, r.y1);
    if p.x < r.x0 || p.x > r.x1 || p.y < r.y0 || p.y > r.y1 {
        return Point::new(cx, cy);
    }
    let dt = (p.y - r.y0).abs();
    let db = (p.y - r.y1).abs();
    let dl = (p.x - r.x0).abs();
    let dr = (p.x - r.x1).abs();
    let min = dt.min(db).min(dl).min(dr);
    if min == dt {
        Point::new(p.x, r.y0)
    } else if min == db {
        Point::new(p.x, r.y1)
    } else if min == dl {
        Point::new(r.x0, p.y)
    } else {
        Point::new(r.x1, p.y)
    }
}

pub(crate) fn classify_border_point(p: Point, b: Rect) -> (BindSide, f64) {
    let c = b.center();
    let hw = (b.width() / 2.0).max(f64::EPSILON);
    let hh = (b.height() / 2.0).max(f64::EPSILON);
    let dt = (p.y - b.y0).abs();
    let db = (p.y - b.y1).abs();
    let dl = (p.x - b.x0).abs();
    let dr = (p.x - b.x1).abs();
    let min = dt.min(db).min(dl).min(dr);
    if min == dt {
        (BindSide::Top, ((p.x - c.x) / hw).clamp(-1.0, 1.0))
    } else if min == db {
        (BindSide::Bottom, ((p.x - c.x) / hw).clamp(-1.0, 1.0))
    } else if min == dl {
        (BindSide::Left, ((p.y - c.y) / hh).clamp(-1.0, 1.0))
    } else {
        (BindSide::Right, ((p.y - c.y) / hh).clamp(-1.0, 1.0))
    }
}

pub fn rect_midpoints(b: Rect) -> [Point; 4] {
    let c = b.center();
    [
        Point::new(c.x, b.y0),
        Point::new(b.x1, c.y),
        Point::new(c.x, b.y1),
        Point::new(b.x0, c.y),
    ]
}
