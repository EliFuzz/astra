use crate::canvas::Canvas;
use crate::selection::HandleKind;
use crate::shapes::{ArrowBinding, BindSide, Shape, ShapeId};
use crate::snap::{ARROW_BIND_BORDER_RADIUS, ARROW_BIND_MIDPOINT_RADIUS};
use kurbo::{Point, Rect};

const MAX_SNAP_CANDIDATES: usize = 200;
const MAX_ROUTING_OBSTACLES: usize = 200;

pub(super) fn self_snap_rects(shape: &Shape, handle: Option<HandleKind>) -> Vec<Rect> {
    let points = shape.snap_points();
    if points.is_empty() {
        return Vec::new();
    }
    let dragged_idx = match handle {
        Some(HandleKind::Endpoint(idx)) => Some(idx),
        Some(HandleKind::IntermediatePoint(idx)) => Some(idx + 1),
        _ => None,
    };
    points
        .iter()
        .enumerate()
        .filter(|(i, _)| dragged_idx.is_none_or(|d| *i != d))
        .map(|(_, pt)| Rect::new(pt.x, pt.y, pt.x, pt.y))
        .collect()
}

pub(super) fn collect_snap_candidates(
    canvas: &Canvas,
    exclude_ids: &[ShapeId],
    snap_zone: Option<Rect>,
) -> Vec<Rect> {
    let viewport = canvas.visible_world_bounds();
    let mut rects: Vec<Rect> = Vec::new();
    for shape in canvas
        .document
        .shapes_ordered()
        .filter(|s| !exclude_ids.contains(&s.id()))
        .filter(|s| {
            !viewport
                .intersect(s.bounds().inflate(1.0, 1.0))
                .is_zero_area()
        })
        .filter(|s| {
            snap_zone.is_none_or(|z| !z.intersect(s.bounds().inflate(1.0, 1.0)).is_zero_area())
        })
        .take(MAX_SNAP_CANDIDATES)
    {
        rects.push(shape.bounds());
        for pt in shape.snap_points() {
            rects.push(Rect::new(pt.x, pt.y, pt.x, pt.y));
        }
    }
    rects
}

pub(super) fn get_line_other_endpoint(shape: &Shape, handle: Option<HandleKind>) -> Point {
    match shape {
        Shape::Line(line) => match handle {
            Some(HandleKind::Endpoint(0)) => line.end,
            Some(HandleKind::Endpoint(1)) => line.start,
            _ => line.start,
        },
        Shape::Arrow(arrow) => match handle {
            Some(HandleKind::Endpoint(0)) => arrow.end,
            Some(HandleKind::Endpoint(1)) => arrow.start,
            _ => arrow.start,
        },
        _ => Point::ZERO,
    }
}

pub(super) fn find_shape_binding_snap(
    canvas: &Canvas,
    point: Point,
    exclude_ids: &[ShapeId],
) -> Option<(ShapeId, Point, BindSide, f64)> {
    let viewport = canvas.visible_world_bounds().inflate(100.0, 100.0);
    for shape in canvas
        .document
        .shapes_ordered()
        .filter(|s| !exclude_ids.contains(&s.id()))
        .filter(|s| {
            !viewport
                .intersect(s.bounds().inflate(1.0, 1.0))
                .is_zero_area()
        })
    {
        if let Some((snap_pt, side, focus)) = shape.find_arrow_binding_snap(
            point,
            ARROW_BIND_MIDPOINT_RADIUS,
            ARROW_BIND_BORDER_RADIUS,
        ) {
            return Some((shape.id(), snap_pt, side, focus));
        }
    }
    None
}

pub(super) fn make_arrow_binding(target_id: ShapeId, side: BindSide, focus: f64) -> ArrowBinding {
    ArrowBinding {
        target_id,
        side,
        focus,
    }
}

pub(super) fn collect_routing_obstacles(canvas: &Canvas, exclude_ids: &[ShapeId]) -> Vec<Rect> {
    let viewport = canvas.visible_world_bounds().inflate(200.0, 200.0);
    canvas
        .document
        .shapes_ordered()
        .filter(|s| !exclude_ids.contains(&s.id()))
        .filter(|s| {
            matches!(
                s,
                Shape::Rectangle(_)
                    | Shape::Diamond(_)
                    | Shape::Ellipse(_)
                    | Shape::Text(_)
                    | Shape::Math(_)
                    | Shape::Image(_)
            )
        })
        .filter(|s| {
            !viewport
                .intersect(s.bounds().inflate(1.0, 1.0))
                .is_zero_area()
        })
        .take(MAX_ROUTING_OBSTACLES)
        .map(|s| s.bounds())
        .collect()
}
