use super::super::super::types::{SmartGuide, SmartGuideKind, SmartGuideResult};
use super::snap_points as aligned_snap;
use kurbo::{Point, Rect};

pub fn detect_smart_guides_for_point(
    point: Point,
    other_bounds: &[Rect],
    threshold: f64,
) -> SmartGuideResult {
    let mut result = SmartGuideResult {
        point,
        guides: Vec::new(),
        snapped_x: false,
        snapped_y: false,
    };

    let mut best_dx: Option<(f64, f64, f64, f64)> = None;
    let mut best_dy: Option<(f64, f64, f64, f64)> = None;
    let mut best_dist_x = threshold;
    let mut best_dist_y = threshold;

    for other in other_bounds {
        let other_cx = (other.x0 + other.x1) / 2.0;
        let other_cy = (other.y0 + other.y1) / 2.0;

        for other_x in [other.x0, other.x1, other_cx] {
            let dist = (point.x - other_x).abs();
            if dist < best_dist_x {
                best_dist_x = dist;
                best_dx = Some((other_x, other_x, other.y0, other.y1));
            }
        }

        for other_y in [other.y0, other.y1, other_cy] {
            let dist = (point.y - other_y).abs();
            if dist < best_dist_y {
                best_dist_y = dist;
                best_dy = Some((other_y, other_y, other.x0, other.x1));
            }
        }
    }

    if let Some((snap_x, guide_x, other_y0, other_y1)) = best_dx {
        result.point.x = snap_x;
        result.snapped_x = true;
        let min_y = point.y.min(other_y0).min(other_y1);
        let max_y = point.y.max(other_y0).max(other_y1);

        let mut snap_points = vec![point.y];
        snap_points.extend(aligned_snap::collect_for_vertical_guide(guide_x, other_bounds));

        result.guides.push(SmartGuide {
            kind: SmartGuideKind::Vertical,
            position: guide_x,
            start: min_y,
            end: max_y,
            snap_points,
        });
    }

    if let Some((snap_y, guide_y, other_x0, other_x1)) = best_dy {
        result.point.y = snap_y;
        result.snapped_y = true;
        let snapped_x = if result.snapped_x {
            result.point.x
        } else {
            point.x
        };
        let min_x = snapped_x.min(other_x0).min(other_x1);
        let max_x = snapped_x.max(other_x0).max(other_x1);

        let mut snap_points = vec![snapped_x];
        snap_points.extend(aligned_snap::collect_for_horizontal_guide(guide_y, other_bounds));

        result.guides.push(SmartGuide {
            kind: SmartGuideKind::Horizontal,
            position: guide_y,
            start: min_x,
            end: max_x,
            snap_points,
        });
    }

    result
}
