use super::Group;
use astra_core::ShapeTrait;
use crate::shapes::{Rectangle, Shape};
use kurbo::Point;

#[test]
fn test_group_creation() {
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect2 = Rectangle::new(Point::new(200.0, 200.0), 50.0, 100.0);

    let group = Group::new(vec![Shape::Rectangle(rect1), Shape::Rectangle(rect2)]);

    assert_eq!(group.children().len(), 2);
}

#[test]
fn test_group_bounds() {
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect2 = Rectangle::new(Point::new(200.0, 200.0), 50.0, 100.0);

    let group = Group::new(vec![Shape::Rectangle(rect1), Shape::Rectangle(rect2)]);
    let bounds = group.bounds();

    assert!((bounds.x0 - 0.0).abs() < f64::EPSILON);
    assert!((bounds.y0 - 0.0).abs() < f64::EPSILON);
    assert!((bounds.x1 - 250.0).abs() < f64::EPSILON);
    assert!((bounds.y1 - 300.0).abs() < f64::EPSILON);
}

#[test]
fn test_group_hit_test() {
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect2 = Rectangle::new(Point::new(200.0, 200.0), 50.0, 100.0);

    let group = Group::new(vec![Shape::Rectangle(rect1), Shape::Rectangle(rect2)]);

    assert!(group.hit_test(Point::new(50.0, 25.0), 0.0));
    assert!(group.hit_test(Point::new(225.0, 250.0), 0.0));
    assert!(!group.hit_test(Point::new(150.0, 100.0), 0.0));
}

#[test]
fn test_nested_groups() {
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect2 = Rectangle::new(Point::new(200.0, 200.0), 50.0, 100.0);

    let inner_group = Group::new(vec![Shape::Rectangle(rect1)]);
    let outer_group = Group::new(vec![Shape::Group(inner_group), Shape::Rectangle(rect2)]);

    assert!(outer_group.hit_test(Point::new(50.0, 25.0), 0.0));
}

#[test]
fn test_ungroup() {
    let rect1 = Rectangle::new(Point::new(0.0, 0.0), 100.0, 50.0);
    let rect2 = Rectangle::new(Point::new(200.0, 200.0), 50.0, 100.0);

    let group = Group::new(vec![Shape::Rectangle(rect1), Shape::Rectangle(rect2)]);
    let children = group.ungroup();

    assert_eq!(children.len(), 2);
}
