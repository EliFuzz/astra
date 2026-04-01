use super::types::SnapResult;
use kurbo::Point;

pub fn snap_to_grid(point: Point, grid_size: f64) -> SnapResult {
    let snapped_x = (point.x / grid_size).round() * grid_size;
    let snapped_y = (point.y / grid_size).round() * grid_size;

    SnapResult {
        point: Point::new(snapped_x, snapped_y),
        snapped_x: true,
        snapped_y: true,
    }
}

pub fn snap_point(point: Point, grid_enabled: bool, grid_size: f64) -> SnapResult {
    if grid_enabled {
        snap_to_grid(point, grid_size)
    } else {
        SnapResult::none(point)
    }
}

pub fn snap_ray_to_grid_lines(
    origin: Point,
    angle_degrees: f64,
    target_point: Point,
    grid_size: f64,
) -> SnapResult {
    let angle_rad = angle_degrees.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    let mut best_point = target_point;
    let mut best_dist_to_target = f64::MAX;
    let mut found = false;

    let is_horizontal = sin_a.abs() < 0.001;
    let is_vertical = cos_a.abs() < 0.001;

    if is_horizontal {
        let y = origin.y;
        let base_grid_x = (target_point.x / grid_size).round() as i32;
        for dx in -5..=5 {
            let grid_x = (base_grid_x + dx) as f64 * grid_size;
            if (grid_x - origin.x) * cos_a >= 0.0 || (grid_x - origin.x).abs() < 0.001 {
                let candidate = Point::new(grid_x, y);
                let dist =
                    (candidate.x - target_point.x).powi(2) + (candidate.y - target_point.y).powi(2);
                if dist < best_dist_to_target {
                    best_dist_to_target = dist;
                    best_point = candidate;
                    found = true;
                }
            }
        }
    } else if is_vertical {
        let x = origin.x;
        let base_grid_y = (target_point.y / grid_size).round() as i32;
        for dy in -5..=5 {
            let grid_y = (base_grid_y + dy) as f64 * grid_size;
            if (grid_y - origin.y) * sin_a >= 0.0 || (grid_y - origin.y).abs() < 0.001 {
                let candidate = Point::new(x, grid_y);
                let dist =
                    (candidate.x - target_point.x).powi(2) + (candidate.y - target_point.y).powi(2);
                if dist < best_dist_to_target {
                    best_dist_to_target = dist;
                    best_point = candidate;
                    found = true;
                }
            }
        }
    } else {
        let base_grid_x = (target_point.x / grid_size).round() as i32;
        for dx in -5..=5 {
            let grid_x = (base_grid_x + dx) as f64 * grid_size;
            let t = (grid_x - origin.x) / cos_a;
            if t >= 0.0 {
                let y = origin.y + t * sin_a;
                let candidate = Point::new(grid_x, y);
                let dist =
                    (candidate.x - target_point.x).powi(2) + (candidate.y - target_point.y).powi(2);
                if dist < best_dist_to_target {
                    best_dist_to_target = dist;
                    best_point = candidate;
                    found = true;
                }
            }
        }

        let base_grid_y = (target_point.y / grid_size).round() as i32;
        for dy in -5..=5 {
            let grid_y = (base_grid_y + dy) as f64 * grid_size;
            let t = (grid_y - origin.y) / sin_a;
            if t >= 0.0 {
                let x = origin.x + t * cos_a;
                let candidate = Point::new(x, grid_y);
                let dist =
                    (candidate.x - target_point.x).powi(2) + (candidate.y - target_point.y).powi(2);
                if dist < best_dist_to_target {
                    best_dist_to_target = dist;
                    best_point = candidate;
                    found = true;
                }
            }
        }
    }

    if found {
        SnapResult {
            point: best_point,
            snapped_x: true,
            snapped_y: true,
        }
    } else {
        SnapResult::none(target_point)
    }
}
