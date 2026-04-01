use super::types::{Corner, Handle, HandleKind};
use crate::shapes::Shape;
use kurbo::{Point, Rect};

pub fn get_handles(shape: &Shape) -> Vec<Handle> {
    match shape {
        Shape::Line(line) => polyline_handles(line.start, line.end, &line.intermediate_points),
        Shape::Arrow(arrow) => polyline_handles(arrow.start, arrow.end, &arrow.intermediate_points),
        Shape::Freehand(_) => corner_handles(shape.bounds()),
        _ => corner_and_rotate_handles(shape.bounds(), shape.rotation()),
    }
}

fn polyline_handles(start: Point, end: Point, intermediate_points: &[Point]) -> Vec<Handle> {
    let mut all_pts = Vec::with_capacity(intermediate_points.len() + 2);
    all_pts.push(start);
    all_pts.extend_from_slice(intermediate_points);
    all_pts.push(end);

    let mut handles = Vec::with_capacity(all_pts.len() * 2 + 1);
    handles.push(Handle::new(start, HandleKind::Endpoint(0)));
    for (i, &pt) in intermediate_points.iter().enumerate() {
        handles.push(Handle::new(pt, HandleKind::IntermediatePoint(i)));
    }
    handles.push(Handle::new(end, HandleKind::Endpoint(1)));
    for i in 0..all_pts.len() - 1 {
        let mid = Point::new(
            (all_pts[i].x + all_pts[i + 1].x) / 2.0,
            (all_pts[i].y + all_pts[i + 1].y) / 2.0,
        );
        handles.push(Handle::new(mid, HandleKind::SegmentMidpoint(i)));
    }
    handles
}

fn corner_handles(bounds: Rect) -> Vec<Handle> {
    vec![
        Handle::new(
            Point::new(bounds.x0, bounds.y0),
            HandleKind::Corner(Corner::TopLeft),
        ),
        Handle::new(
            Point::new(bounds.x1, bounds.y0),
            HandleKind::Corner(Corner::TopRight),
        ),
        Handle::new(
            Point::new(bounds.x0, bounds.y1),
            HandleKind::Corner(Corner::BottomLeft),
        ),
        Handle::new(
            Point::new(bounds.x1, bounds.y1),
            HandleKind::Corner(Corner::BottomRight),
        ),
    ]
}

pub const ROTATE_HANDLE_OFFSET: f64 = 25.0;

fn corner_and_rotate_handles(bounds: Rect, rotation: f64) -> Vec<Handle> {
    let center = bounds.center();
    let half_w = bounds.width() / 2.0;
    let half_h = bounds.height() / 2.0;

    let rotate_point = |dx: f64, dy: f64| -> Point {
        let cos_r = rotation.cos();
        let sin_r = rotation.sin();
        Point::new(
            center.x + dx * cos_r - dy * sin_r,
            center.y + dx * sin_r + dy * cos_r,
        )
    };

    vec![
        Handle::new(
            rotate_point(-half_w, -half_h),
            HandleKind::Corner(Corner::TopLeft),
        ),
        Handle::new(
            rotate_point(half_w, -half_h),
            HandleKind::Corner(Corner::TopRight),
        ),
        Handle::new(
            rotate_point(-half_w, half_h),
            HandleKind::Corner(Corner::BottomLeft),
        ),
        Handle::new(
            rotate_point(half_w, half_h),
            HandleKind::Corner(Corner::BottomRight),
        ),
        Handle::new(
            rotate_point(0.0, -half_h - ROTATE_HANDLE_OFFSET),
            HandleKind::Rotate,
        ),
    ]
}

pub fn hit_test_handles(shape: &Shape, point: Point, tolerance: f64) -> Option<HandleKind> {
    let handles = get_handles(shape);
    for handle in handles {
        if handle.hit_test(point, tolerance) {
            return Some(handle.kind);
        }
    }
    None
}

pub fn hit_test_boundary(shape: &Shape, point: Point, tolerance: f64) -> bool {
    let bounds = shape.bounds();
    let outer = bounds.inflate(tolerance, tolerance);
    let inner = bounds.inset(
        tolerance
            .min(bounds.width() / 2.0)
            .min(bounds.height() / 2.0),
    );
    outer.contains(point) && !inner.contains(point)
}
