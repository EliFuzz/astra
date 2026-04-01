use super::super::types::{SmartGuide, SmartGuideKind, SmartGuideResult};
use kurbo::{Point, Rect};

pub fn snap_ray_to_smart_guides(
    origin: Point,
    angle_degrees: f64,
    target_point: Point,
    other_bounds: &[Rect],
    threshold: f64,
) -> SmartGuideResult {
    let angle_rad = angle_degrees.to_radians();
    let cos_a = angle_rad.cos();
    let sin_a = angle_rad.sin();

    let mut result = SmartGuideResult {
        point: target_point,
        guides: Vec::new(),
        snapped_x: false,
        snapped_y: false,
    };

    let mut best_t: Option<(f64, Point, SmartGuide)> = None;
    let mut best_dist = f64::MAX;

    for other in other_bounds {
        let other_cx = (other.x0 + other.x1) / 2.0;
        let other_cy = (other.y0 + other.y1) / 2.0;

        if cos_a.abs() > 0.001 {
            for other_x in [other.x0, other.x1, other_cx] {
                let t = (other_x - origin.x) / cos_a;
                if t > 0.0 {
                    let intersect_y = origin.y + t * sin_a;
                    let intersect = Point::new(other_x, intersect_y);
                    let dist = ((intersect.x - target_point.x).powi(2)
                        + (intersect.y - target_point.y).powi(2))
                    .sqrt();
                    if dist < threshold && dist < best_dist {
                        best_dist = dist;
                        let min_y = intersect_y.min(other.y0).min(other.y1);
                        let max_y = intersect_y.max(other.y0).max(other.y1);
                        best_t = Some((
                            t,
                            intersect,
                            SmartGuide {
                                kind: SmartGuideKind::Vertical,
                                position: other_x,
                                start: min_y,
                                end: max_y,
                                snap_points: vec![intersect_y, other.y0, other.y1, other_cy],
                            },
                        ));
                    }
                }
            }
        }

        if sin_a.abs() > 0.001 {
            for other_y in [other.y0, other.y1, other_cy] {
                let t = (other_y - origin.y) / sin_a;
                if t > 0.0 {
                    let intersect_x = origin.x + t * cos_a;
                    let intersect = Point::new(intersect_x, other_y);
                    let dist = ((intersect.x - target_point.x).powi(2)
                        + (intersect.y - target_point.y).powi(2))
                    .sqrt();
                    if dist < threshold && dist < best_dist {
                        best_dist = dist;
                        let min_x = intersect_x.min(other.x0).min(other.x1);
                        let max_x = intersect_x.max(other.x0).max(other.x1);
                        best_t = Some((
                            t,
                            intersect,
                            SmartGuide {
                                kind: SmartGuideKind::Horizontal,
                                position: other_y,
                                start: min_x,
                                end: max_x,
                                snap_points: vec![intersect_x, other.x0, other.x1, other_cx],
                            },
                        ));
                    }
                }
            }
        }
    }

    if let Some((_, point, guide)) = best_t {
        result.point = point;
        result.snapped_x = true;
        result.snapped_y = true;
        result.guides.push(guide);
    }

    result
}
