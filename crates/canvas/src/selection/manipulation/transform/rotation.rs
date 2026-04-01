use crate::shapes::Shape;
use kurbo::Point;

pub fn apply_rotation(shape: &mut Shape, cursor_point: Point, snap_to_15deg: bool) -> f64 {
    let bounds = shape.bounds();
    let center = bounds.center();

    let dx = cursor_point.x - center.x;
    let dy = cursor_point.y - center.y;
    let mut angle = dy.atan2(dx) + std::f64::consts::FRAC_PI_2;

    if snap_to_15deg {
        let snap_angle = std::f64::consts::PI / 12.0;
        angle = (angle / snap_angle).round() * snap_angle;
    }

    shape.set_rotation(angle);
    angle
}

pub fn reset_rotation(shape: &mut Shape, angle_degrees: f64) {
    let angle_radians = angle_degrees.to_radians();
    shape.set_rotation(angle_radians);
}
