use super::super::super::types::{SmartGuide, SmartGuideKind, SmartGuideResult};
use super::super::spacing::{detect_equal_spacing_h, detect_equal_spacing_v};
use super::snap_points as aligned_snap;
use kurbo::{Point, Rect};

pub fn detect_smart_guides(
    dragged_bounds: Rect,
    other_bounds: &[Rect],
    threshold: f64,
) -> SmartGuideResult {
    let shape_bounds: Vec<&Rect> = other_bounds.iter().filter(|r| !r.is_zero_area()).collect();

    let mut result = SmartGuideResult {
        point: Point::new(dragged_bounds.x0, dragged_bounds.y0),
        guides: Vec::new(),
        snapped_x: false,
        snapped_y: false,
    };

    let dragged_cx = (dragged_bounds.x0 + dragged_bounds.x1) / 2.0;
    let dragged_cy = (dragged_bounds.y0 + dragged_bounds.y1) / 2.0;
    let dragged_w = dragged_bounds.x1 - dragged_bounds.x0;
    let dragged_h = dragged_bounds.y1 - dragged_bounds.y0;

    let mut best_dx: Option<(f64, f64, Rect)> = None;
    let mut best_dy: Option<(f64, f64, Rect)> = None;
    let mut best_dist_x = threshold;
    let mut best_dist_y = threshold;

    for other in other_bounds {
        let other_cx = (other.x0 + other.x1) / 2.0;
        let other_cy = (other.y0 + other.y1) / 2.0;

        for dragged_x in [dragged_bounds.x0, dragged_bounds.x1, dragged_cx] {
            for other_x in [other.x0, other.x1, other_cx] {
                let dist = (dragged_x - other_x).abs();
                if dist < best_dist_x {
                    best_dist_x = dist;
                    let snap_x = dragged_bounds.x0 + (other_x - dragged_x);
                    best_dx = Some((snap_x, other_x, *other));
                }
            }
        }

        for dragged_y in [dragged_bounds.y0, dragged_bounds.y1, dragged_cy] {
            for other_y in [other.y0, other.y1, other_cy] {
                let dist = (dragged_y - other_y).abs();
                if dist < best_dist_y {
                    best_dist_y = dist;
                    let snap_y = dragged_bounds.y0 + (other_y - dragged_y);
                    best_dy = Some((snap_y, other_y, *other));
                }
            }
        }
    }

    let max_spacing_candidates = 50;
    if shape_bounds.len() <= max_spacing_candidates {
        let h_candidates: Vec<&Rect> = shape_bounds
            .iter()
            .filter(|b| b.y0 < dragged_bounds.y1 && b.y1 > dragged_bounds.y0)
            .copied()
            .collect();
        let v_candidates: Vec<&Rect> = shape_bounds
            .iter()
            .filter(|b| b.x0 < dragged_bounds.x1 && b.x1 > dragged_bounds.x0)
            .copied()
            .collect();

        detect_equal_spacing_h(
            &h_candidates,
            &dragged_bounds,
            dragged_cx,
            dragged_cy,
            dragged_w,
            &mut best_dist_x,
            &mut best_dx,
            &mut result,
        );

        detect_equal_spacing_v(
            &v_candidates,
            &dragged_bounds,
            dragged_cx,
            dragged_cy,
            dragged_h,
            &mut best_dist_y,
            &mut best_dy,
            &mut result,
        );
    }

    if let Some((snap_x, guide_x, other)) = best_dx {
        result.point.x = snap_x;
        result.snapped_x = true;
        let snapped_y0 = dragged_bounds.y0;
        let snapped_y1 = dragged_bounds.y1;
        let min_y = snapped_y0.min(snapped_y1).min(other.y0).min(other.y1);
        let max_y = snapped_y0.max(snapped_y1).max(other.y0).max(other.y1);

        let mut snap_points = vec![snapped_y0, snapped_y1, dragged_cy];
        snap_points.extend(aligned_snap::collect_for_vertical_guide(guide_x, other_bounds));

        result.guides.push(SmartGuide {
            kind: SmartGuideKind::Vertical,
            position: guide_x,
            start: min_y,
            end: max_y,
            snap_points,
        });
    }

    if let Some((snap_y, guide_y, other)) = best_dy {
        result.point.y = snap_y;
        result.snapped_y = true;
        let snapped_x0 = if result.snapped_x {
            result.point.x
        } else {
            dragged_bounds.x0
        };
        let snapped_x1 = snapped_x0 + dragged_w;
        let min_x = snapped_x0.min(snapped_x1).min(other.x0).min(other.x1);
        let max_x = snapped_x0.max(snapped_x1).max(other.x0).max(other.x1);

        let mut snap_points = vec![snapped_x0, snapped_x1, snapped_x0 + dragged_w / 2.0];
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
