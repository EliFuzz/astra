use super::super::super::EQUAL_SPACING_SNAP_RADIUS;
use super::super::super::types::{SmartGuide, SmartGuideKind, SmartGuideResult};
use kurbo::Rect;

#[derive(Clone, Copy)]
pub(super) enum SpacingAxis {
    Horizontal,
    Vertical,
}

impl SpacingAxis {
    fn kind(self) -> SmartGuideKind {
        match self {
            SpacingAxis::Horizontal => SmartGuideKind::EqualSpacingH,
            SpacingAxis::Vertical => SmartGuideKind::EqualSpacingV,
        }
    }

    fn leading(self, r: &Rect) -> f64 {
        match self {
            SpacingAxis::Horizontal => r.x0,
            SpacingAxis::Vertical => r.y0,
        }
    }

    fn trailing(self, r: &Rect) -> f64 {
        match self {
            SpacingAxis::Horizontal => r.x1,
            SpacingAxis::Vertical => r.y1,
        }
    }

    fn dragged_origin(self, r: &Rect) -> f64 {
        self.leading(r)
    }

    fn primary_fixed(self, dragged_cx: f64, dragged_cy: f64) -> f64 {
        match self {
            SpacingAxis::Horizontal => dragged_cy,
            SpacingAxis::Vertical => dragged_cx,
        }
    }

    fn cross_mid(self, r: &Rect) -> f64 {
        match self {
            SpacingAxis::Horizontal => (r.y0 + r.y1) / 2.0,
            SpacingAxis::Vertical => (r.x0 + r.x1) / 2.0,
        }
    }

    fn cross_mid_min(self, a: &Rect, b: &Rect) -> f64 {
        self.cross_mid(a).min(self.cross_mid(b))
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn detect_equal_spacing(
    axis: SpacingAxis,
    candidates: &[&Rect],
    dragged: &Rect,
    dragged_cx: f64,
    dragged_cy: f64,
    dragged_len: f64,
    best_dist: &mut f64,
    best_d: &mut Option<(f64, f64, Rect)>,
    result: &mut SmartGuideResult,
) {
    let kind = axis.kind();
    let fixed = axis.primary_fixed(dragged_cx, dragged_cy);

    let mut sorted: Vec<&Rect> = candidates.to_vec();
    sorted.sort_by(|a, b| {
        axis
            .leading(a)
            .partial_cmp(&axis.leading(b))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut best_spacing: Option<f64> = None;
    let mut best_lo_idx: Option<usize> = None;
    let mut best_hi_idx: Option<usize> = None;

    for i in 0..sorted.len() {
        for j in (i + 1)..sorted.len() {
            let (lo, hi) = (sorted[i], sorted[j]);
            if axis.trailing(lo) >= axis.leading(hi) {
                continue;
            }
            let gap = axis.leading(hi) - axis.trailing(lo);
            if gap > EQUAL_SPACING_SNAP_RADIUS {
                continue;
            }

            if gap > dragged_len {
                let target = axis.trailing(lo) + (gap - dragged_len) / 2.0;
                let dist = (axis.dragged_origin(dragged) - target).abs();
                if dist < *best_dist {
                    *best_dist = dist;
                    *best_d = Some((target, target + dragged_len / 2.0, *lo));
                    best_spacing = Some((gap - dragged_len) / 2.0);
                    best_lo_idx = Some(i);
                    best_hi_idx = Some(j);
                    result.guides.retain(|g| g.kind != kind);
                    result.guides.push(SmartGuide {
                        kind,
                        position: fixed,
                        start: axis.trailing(lo),
                        end: target,
                        snap_points: vec![],
                    });
                    result.guides.push(SmartGuide {
                        kind,
                        position: fixed,
                        start: target + dragged_len,
                        end: axis.leading(hi),
                        snap_points: vec![],
                    });
                }
            }

            let target_hi = axis.trailing(hi) + gap;
            let dist_hi = (axis.dragged_origin(dragged) - target_hi).abs();
            if dist_hi < *best_dist {
                *best_dist = dist_hi;
                *best_d = Some((target_hi, target_hi + dragged_len / 2.0, *hi));
                best_spacing = Some(gap);
                best_lo_idx = Some(i);
                best_hi_idx = Some(j);
                result.guides.retain(|g| g.kind != kind);
                let cross = axis.cross_mid(lo);
                result.guides.push(SmartGuide {
                    kind,
                    position: cross,
                    start: axis.trailing(lo),
                    end: axis.leading(hi),
                    snap_points: vec![],
                });
                result.guides.push(SmartGuide {
                    kind,
                    position: fixed,
                    start: axis.trailing(hi),
                    end: target_hi,
                    snap_points: vec![],
                });
            }

            let target_lo = axis.leading(lo) - gap - dragged_len;
            let dist_lo = (axis.dragged_origin(dragged) - target_lo).abs();
            if dist_lo < *best_dist {
                *best_dist = dist_lo;
                *best_d = Some((target_lo, target_lo + dragged_len / 2.0, *lo));
                best_spacing = Some(gap);
                best_lo_idx = Some(i);
                best_hi_idx = Some(j);
                result.guides.retain(|g| g.kind != kind);
                let cross = axis.cross_mid(hi);
                result.guides.push(SmartGuide {
                    kind,
                    position: fixed,
                    start: target_lo + dragged_len,
                    end: axis.leading(lo),
                    snap_points: vec![],
                });
                result.guides.push(SmartGuide {
                    kind,
                    position: cross,
                    start: axis.trailing(lo),
                    end: axis.leading(hi),
                    snap_points: vec![],
                });
            }
        }
    }

    if let (Some(spacing), Some(li), Some(ri)) = (best_spacing, best_lo_idx, best_hi_idx) {
        let mut cur = sorted[li];
        for k in (0..li).rev() {
            let candidate = sorted[k];
            if (axis.trailing(candidate) - (axis.leading(cur) - spacing)).abs() < 1.0 {
                let pos = axis.cross_mid_min(candidate, cur);
                result.guides.push(SmartGuide {
                    kind,
                    position: pos,
                    start: axis.trailing(candidate),
                    end: axis.leading(cur),
                    snap_points: vec![],
                });
                cur = candidate;
            }
        }
        cur = sorted[ri];
        for candidate in &sorted[(ri + 1)..] {
            if (axis.leading(candidate) - (axis.trailing(cur) + spacing)).abs() < 1.0 {
                let pos = axis.cross_mid_min(candidate, cur);
                result.guides.push(SmartGuide {
                    kind,
                    position: pos,
                    start: axis.trailing(cur),
                    end: axis.leading(candidate),
                    snap_points: vec![],
                });
                cur = candidate;
            }
        }
    }
}
