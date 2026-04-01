use crate::shapes::ShapeTrait;
use kurbo::Point;

use super::Line;

#[test]
fn test_line_creation() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!((line.length() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_midpoint() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0));
    let mid = line.midpoint();
    assert!((mid.x - 50.0).abs() < f64::EPSILON);
    assert!((mid.y - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_hit_test_on_line() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!(line.hit_test(Point::new(50.0, 0.0), 1.0));
    assert!(line.hit_test(Point::new(50.0, 2.0), 5.0));
    assert!(!line.hit_test(Point::new(50.0, 20.0), 5.0));
}

#[test]
fn test_hit_test_endpoints() {
    let line = Line::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!(line.hit_test(Point::new(0.0, 0.0), 1.0));
    assert!(line.hit_test(Point::new(100.0, 0.0), 1.0));
}

#[test]
fn test_bounds() {
    let line = Line::new(Point::new(10.0, 20.0), Point::new(50.0, 80.0));
    let bounds = line.bounds();
    assert!((bounds.x0 - 10.0).abs() < f64::EPSILON);
    assert!((bounds.y0 - 20.0).abs() < f64::EPSILON);
    assert!((bounds.x1 - 50.0).abs() < f64::EPSILON);
    assert!((bounds.y1 - 80.0).abs() < f64::EPSILON);
}
