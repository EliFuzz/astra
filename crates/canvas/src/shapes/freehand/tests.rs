use super::Freehand;
use crate::shapes::ShapeTrait;
use kurbo::Point;

#[test]
fn test_freehand_creation() {
    let freehand = Freehand::new();
    assert!(freehand.is_empty());
}

#[test]
fn test_add_points() {
    let mut freehand = Freehand::new();
    freehand.add_point(Point::new(0.0, 0.0));
    freehand.add_point(Point::new(10.0, 10.0));
    assert_eq!(freehand.len(), 2);
}

#[test]
fn test_bounds() {
    let freehand = Freehand::from_points(vec![
        Point::new(0.0, 0.0),
        Point::new(100.0, 50.0),
        Point::new(50.0, 100.0),
    ]);

    let bounds = freehand.bounds();
    assert!((bounds.x0).abs() < f64::EPSILON);
    assert!((bounds.y0).abs() < f64::EPSILON);
    assert!((bounds.x1 - 100.0).abs() < f64::EPSILON);
    assert!((bounds.y1 - 100.0).abs() < f64::EPSILON);
}

#[test]
fn test_simplify() {
    let mut freehand = Freehand::from_points(vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.1),
        Point::new(2.0, 0.0),
        Point::new(3.0, 0.1),
        Point::new(4.0, 0.0),
    ]);

    freehand.simplify(0.5);
    assert!(freehand.len() < 5);
}

#[test]
fn test_hit_test() {
    let freehand = Freehand::from_points(vec![Point::new(0.0, 0.0), Point::new(100.0, 0.0)]);

    assert!(freehand.hit_test(Point::new(50.0, 0.0), 5.0));
    assert!(!freehand.hit_test(Point::new(50.0, 20.0), 5.0));
}
