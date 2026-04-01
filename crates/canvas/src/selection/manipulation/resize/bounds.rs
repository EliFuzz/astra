use super::super::super::types::Corner;
use kurbo::{Rect, Vec2};

pub(crate) fn resize_bounds(bounds: Rect, corner: Corner, delta: Vec2) -> (f64, f64, f64, f64) {
    let (raw_x0, raw_y0, raw_x1, raw_y1) = match corner {
        Corner::TopLeft => (
            bounds.x0 + delta.x,
            bounds.y0 + delta.y,
            bounds.x1,
            bounds.y1,
        ),
        Corner::TopRight => (
            bounds.x0,
            bounds.y0 + delta.y,
            bounds.x1 + delta.x,
            bounds.y1,
        ),
        Corner::BottomLeft => (
            bounds.x0 + delta.x,
            bounds.y0,
            bounds.x1,
            bounds.y1 + delta.y,
        ),
        Corner::BottomRight => (
            bounds.x0,
            bounds.y0,
            bounds.x1 + delta.x,
            bounds.y1 + delta.y,
        ),
    };
    (
        raw_x0.min(raw_x1),
        raw_y0.min(raw_y1),
        raw_x0.max(raw_x1),
        raw_y0.max(raw_y1),
    )
}
