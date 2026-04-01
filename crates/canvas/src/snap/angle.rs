use super::grid::{snap_ray_to_grid_lines, snap_to_grid};
use super::types::AngleSnapResult;
use kurbo::Point;

pub const ANGLE_SNAP_INCREMENT: f64 = 15.0;

pub fn snap_angle(angle_degrees: f64, increment: f64) -> f64 {
    let snapped = (angle_degrees / increment).round() * increment;
    if snapped < 0.0 {
        snapped + 360.0
    } else if snapped >= 360.0 {
        snapped - 360.0
    } else {
        snapped
    }
}

pub fn snap_line_endpoint(start: Point, end: Point, angle_snap_enabled: bool) -> AngleSnapResult {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 0.001 {
        return AngleSnapResult {
            point: end,
            angle_degrees: 0.0,
            original_angle_degrees: 0.0,
            snapped: false,
            distance: 0.0,
        };
    }

    let original_angle = dy.atan2(dx).to_degrees();
    let original_angle_normalized = if original_angle < 0.0 {
        original_angle + 360.0
    } else {
        original_angle
    };

    if !angle_snap_enabled {
        return AngleSnapResult {
            point: end,
            angle_degrees: original_angle_normalized,
            original_angle_degrees: original_angle_normalized,
            snapped: false,
            distance,
        };
    }

    let snapped_angle = snap_angle(original_angle_normalized, ANGLE_SNAP_INCREMENT);
    let snapped_radians = snapped_angle.to_radians();

    let snapped_point = Point::new(
        start.x + distance * snapped_radians.cos(),
        start.y + distance * snapped_radians.sin(),
    );

    let angle_diff = (snapped_angle - original_angle_normalized).abs();
    let did_snap = angle_diff > 0.1 && angle_diff < 359.9;

    AngleSnapResult {
        point: snapped_point,
        angle_degrees: snapped_angle,
        original_angle_degrees: original_angle_normalized,
        snapped: did_snap || angle_diff < 0.1,
        distance,
    }
}

pub fn snap_line_endpoint_isometric(
    start: Point,
    end: Point,
    angle_snap_enabled: bool,
    grid_snap_enabled: bool,
    _use_isometric_grid: bool,
    grid_size: f64,
) -> AngleSnapResult {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 0.001 {
        return AngleSnapResult {
            point: end,
            angle_degrees: 0.0,
            original_angle_degrees: 0.0,
            snapped: false,
            distance: 0.0,
        };
    }

    let original_angle = dy.atan2(dx).to_degrees();
    let original_angle_normalized = if original_angle < 0.0 {
        original_angle + 360.0
    } else {
        original_angle
    };

    if angle_snap_enabled && grid_snap_enabled {
        let snapped_angle = snap_angle(original_angle_normalized, ANGLE_SNAP_INCREMENT);
        let snapped_radians = snapped_angle.to_radians();
        let angle_snapped_point = Point::new(
            start.x + distance * snapped_radians.cos(),
            start.y + distance * snapped_radians.sin(),
        );

        let grid_snap =
            snap_ray_to_grid_lines(start, snapped_angle, angle_snapped_point, grid_size);

        if grid_snap.is_snapped() {
            let new_dx = grid_snap.point.x - start.x;
            let new_dy = grid_snap.point.y - start.y;
            let new_distance = (new_dx * new_dx + new_dy * new_dy).sqrt();

            return AngleSnapResult {
                point: grid_snap.point,
                angle_degrees: snapped_angle,
                original_angle_degrees: original_angle_normalized,
                snapped: true,
                distance: new_distance,
            };
        }

        return AngleSnapResult {
            point: angle_snapped_point,
            angle_degrees: snapped_angle,
            original_angle_degrees: original_angle_normalized,
            snapped: true,
            distance,
        };
    }

    if !angle_snap_enabled {
        if grid_snap_enabled {
            let grid_snap = snap_to_grid(end, grid_size);
            return AngleSnapResult {
                point: grid_snap.point,
                angle_degrees: original_angle_normalized,
                original_angle_degrees: original_angle_normalized,
                snapped: grid_snap.is_snapped(),
                distance,
            };
        }
        return AngleSnapResult {
            point: end,
            angle_degrees: original_angle_normalized,
            original_angle_degrees: original_angle_normalized,
            snapped: false,
            distance,
        };
    }

    let snapped_angle = snap_angle(original_angle_normalized, ANGLE_SNAP_INCREMENT);
    let snapped_radians = snapped_angle.to_radians();

    let snapped_point = Point::new(
        start.x + distance * snapped_radians.cos(),
        start.y + distance * snapped_radians.sin(),
    );

    let angle_diff = (snapped_angle - original_angle_normalized).abs();
    let did_snap = angle_diff > 0.1 && angle_diff < 359.9;

    AngleSnapResult {
        point: snapped_point,
        angle_degrees: snapped_angle,
        original_angle_degrees: original_angle_normalized,
        snapped: did_snap || angle_diff < 0.1,
        distance,
    }
}
