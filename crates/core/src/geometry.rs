use kurbo::Point;

pub fn point_to_segment_dist(point: Point, a: Point, b: Point) -> f64 {
    let seg = kurbo::Vec2::new(b.x - a.x, b.y - a.y);
    let pv = kurbo::Vec2::new(point.x - a.x, point.y - a.y);
    let len_sq = seg.hypot2();
    if len_sq < f64::EPSILON {
        return pv.hypot();
    }
    let t = (pv.dot(seg) / len_sq).clamp(0.0, 1.0);
    let proj = Point::new(a.x + t * seg.x, a.y + t * seg.y);
    ((point.x - proj.x).powi(2) + (point.y - proj.y).powi(2)).sqrt()
}

pub fn point_to_polyline_dist(point: Point, points: &[Point]) -> f64 {
    points
        .windows(2)
        .map(|w| point_to_segment_dist(point, w[0], w[1]))
        .fold(f64::INFINITY, f64::min)
}
