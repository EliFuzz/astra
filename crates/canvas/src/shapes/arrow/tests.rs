use super::Arrow;
use crate::shapes::ShapeTrait;
use kurbo::Point;

#[test]
fn test_arrow_creation() {
    let arrow = Arrow::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!((arrow.length() - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_direction() {
    let arrow = Arrow::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    let dir = arrow.direction();
    assert!((dir.x - 1.0).abs() < f64::EPSILON);
    assert!(dir.y.abs() < f64::EPSILON);
}

#[test]
fn test_hit_test_shaft() {
    let arrow = Arrow::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!(arrow.hit_test(Point::new(50.0, 0.0), 5.0));
}

#[test]
fn test_hit_test_head() {
    let arrow = Arrow::new(Point::new(0.0, 0.0), Point::new(100.0, 0.0));
    assert!(arrow.hit_test(Point::new(100.0, 0.0), 1.0));
}
