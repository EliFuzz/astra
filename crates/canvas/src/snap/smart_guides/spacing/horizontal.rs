use super::axis::{SpacingAxis, detect_equal_spacing};
use super::super::super::types::SmartGuideResult;
use kurbo::Rect;

#[allow(clippy::too_many_arguments)]
pub(crate) fn detect_equal_spacing_h(
    candidates: &[&Rect],
    dragged: &Rect,
    dragged_cx: f64,
    dragged_cy: f64,
    dragged_w: f64,
    best_dist: &mut f64,
    best_dx: &mut Option<(f64, f64, Rect)>,
    result: &mut SmartGuideResult,
) {
    detect_equal_spacing(
        SpacingAxis::Horizontal,
        candidates,
        dragged,
        dragged_cx,
        dragged_cy,
        dragged_w,
        best_dist,
        best_dx,
        result,
    );
}
