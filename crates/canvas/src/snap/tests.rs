use super::*;
use kurbo::Point;

#[test]
fn test_snap_to_grid() {
    let result = snap_to_grid(Point::new(23.0, 47.0), 20.0);
    assert_eq!(result.point, Point::new(20.0, 40.0));
    assert!(result.snapped_x);
    assert!(result.snapped_y);
}

#[test]
fn test_snap_to_grid_exact() {
    let result = snap_to_grid(Point::new(40.0, 60.0), 20.0);
    assert_eq!(result.point, Point::new(40.0, 60.0));
}

#[test]
fn test_snap_to_grid_round_up() {
    let result = snap_to_grid(Point::new(31.0, 51.0), 20.0);
    assert_eq!(result.point, Point::new(40.0, 60.0));
}

#[test]
fn test_snap_angle() {
    assert!((snap_angle(0.0, 15.0) - 0.0).abs() < 0.01);
    assert!((snap_angle(7.0, 15.0) - 0.0).abs() < 0.01);
    assert!((snap_angle(8.0, 15.0) - 15.0).abs() < 0.01);
    assert!((snap_angle(22.0, 15.0) - 15.0).abs() < 0.01);
    assert!((snap_angle(23.0, 15.0) - 30.0).abs() < 0.01);
    assert!((snap_angle(45.0, 15.0) - 45.0).abs() < 0.01);
    assert!((snap_angle(90.0, 15.0) - 90.0).abs() < 0.01);
    assert!((snap_angle(180.0, 15.0) - 180.0).abs() < 0.01);
    assert!((snap_angle(270.0, 15.0) - 270.0).abs() < 0.01);
    assert!((snap_angle(359.0, 15.0) - 0.0).abs() < 0.01);
}

#[test]
fn test_snap_line_endpoint_horizontal() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 5.0);
    let result = snap_line_endpoint(start, end, true);

    assert!(result.snapped);
    assert!((result.angle_degrees - 0.0).abs() < 0.01);
    assert!((result.point.y - 0.0).abs() < 0.01);
}

#[test]
fn test_snap_line_endpoint_45_degrees() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 102.0);
    let result = snap_line_endpoint(start, end, true);

    assert!(result.snapped);
    assert!((result.angle_degrees - 45.0).abs() < 0.01);
}

#[test]
fn test_snap_line_endpoint_disabled() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 5.0);
    let result = snap_line_endpoint(start, end, false);

    assert!(!result.snapped);
    assert_eq!(result.point, end);
}

#[test]
fn test_snap_line_preserves_distance() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 10.0);
    let result = snap_line_endpoint(start, end, true);

    let original_distance = ((100.0_f64).powi(2) + (10.0_f64).powi(2)).sqrt();
    assert!((result.distance - original_distance).abs() < 0.01);
}

#[test]
fn test_snap_ray_to_grid_lines_horizontal() {
    let origin = Point::new(0.0, 0.0);
    let target = Point::new(55.0, 0.0);

    let result = snap_ray_to_grid_lines(origin, 0.0, target, 20.0);
    assert!(result.is_snapped());
    assert!((result.point.x - 60.0).abs() < 0.01);
    assert!((result.point.y - 0.0).abs() < 0.01);
}

#[test]
fn test_snap_ray_to_grid_lines_45_degrees() {
    let origin = Point::new(0.0, 0.0);
    let target = Point::new(50.0, 50.0);

    let result = snap_ray_to_grid_lines(origin, 45.0, target, 20.0);
    assert!(result.is_snapped());
    assert!((result.point.x - result.point.y).abs() < 0.01);
}

#[test]
fn test_snap_ray_to_grid_lines_30_degrees() {
    let origin = Point::new(0.0, 0.0);
    let target = Point::new(100.0, 57.7);

    let result = snap_ray_to_grid_lines(origin, 30.0, target, 20.0);
    assert!(result.is_snapped());
    let angle = (result.point.y / result.point.x).atan().to_degrees();
    assert!((angle - 30.0).abs() < 0.1);
}

#[test]
fn test_snap_line_endpoint_isometric() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 5.0);

    let result = snap_line_endpoint_isometric(start, end, true, true, true, 20.0);

    assert!(result.snapped);
    assert!((result.point.y).abs() < 0.01);
    assert!((result.point.x % 20.0).abs() < 0.01 || (result.point.x % 20.0 - 20.0).abs() < 0.01);
}

#[test]
fn test_snap_line_endpoint_isometric_45_degrees() {
    let start = Point::new(0.0, 0.0);
    let end = Point::new(100.0, 102.0);

    let result = snap_line_endpoint_isometric(start, end, true, true, true, 20.0);

    assert!(result.snapped);
    assert!((result.point.x - result.point.y).abs() < 0.1);
    let on_x_grid =
        (result.point.x % 20.0).abs() < 0.01 || (result.point.x % 20.0 - 20.0).abs() < 0.01;
    let on_y_grid =
        (result.point.y % 20.0).abs() < 0.01 || (result.point.y % 20.0 - 20.0).abs() < 0.01;
    assert!(on_x_grid || on_y_grid);
}
