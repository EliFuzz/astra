use kurbo::Point;

pub(super) fn rdp_simplify_with_pressure(
    points: &[Point],
    pressures: &[f64],
    tolerance: f64,
) -> (Vec<Point>, Vec<f64>) {
    if points.len() < 3 {
        let p = if pressures.len() == points.len() {
            pressures.to_vec()
        } else {
            vec![1.0; points.len()]
        };
        return (points.to_vec(), p);
    }

    let first = points[0];
    let last = points[points.len() - 1];

    let mut max_dist = 0.0;
    let mut max_index = 0;

    for (i, point) in points.iter().enumerate().skip(1).take(points.len() - 2) {
        let dist = perpendicular_distance(*point, first, last);
        if dist > max_dist {
            max_dist = dist;
            max_index = i;
        }
    }

    if max_dist > tolerance {
        let left_pressures = if pressures.len() > max_index {
            &pressures[..=max_index]
        } else {
            &[]
        };
        let right_pressures = if pressures.len() > max_index {
            &pressures[max_index..]
        } else {
            &[]
        };

        let (mut left_pts, mut left_press) =
            rdp_simplify_with_pressure(&points[..=max_index], left_pressures, tolerance);
        let (right_pts, right_press) =
            rdp_simplify_with_pressure(&points[max_index..], right_pressures, tolerance);

        left_pts.pop();
        left_press.pop();
        left_pts.extend(right_pts);
        left_press.extend(right_press);
        (left_pts, left_press)
    } else {
        let first_pressure = pressures.first().copied().unwrap_or(1.0);
        let last_pressure = pressures.last().copied().unwrap_or(1.0);
        (vec![first, last], vec![first_pressure, last_pressure])
    }
}

fn perpendicular_distance(point: Point, line_start: Point, line_end: Point) -> f64 {
    let dx = line_end.x - line_start.x;
    let dy = line_end.y - line_start.y;

    let line_len_sq = dx * dx + dy * dy;
    if line_len_sq < f64::EPSILON {
        let px = point.x - line_start.x;
        let py = point.y - line_start.y;
        return (px * px + py * py).sqrt();
    }

    let area2 = ((point.x - line_start.x) * dy - (point.y - line_start.y) * dx).abs();
    area2 / line_len_sq.sqrt()
}
