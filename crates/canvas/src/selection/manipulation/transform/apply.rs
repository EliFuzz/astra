use super::super::resize::{
    apply_corner_resize_diamond, apply_corner_resize_ellipse, apply_corner_resize_freehand,
    apply_corner_resize_group, apply_corner_resize_image, apply_corner_resize_math,
    apply_corner_resize_rect, apply_corner_resize_text,
};
use crate::selection::types::HandleKind;
use crate::shapes::Shape;
use kurbo::Point;

pub fn apply_manipulation(
    shape: &Shape,
    handle: Option<HandleKind>,
    delta: kurbo::Vec2,
    keep_aspect_ratio: bool,
) -> Shape {
    let mut shape = shape.clone();

    match handle {
        None => {
            let translation = kurbo::Affine::translate(delta);
            shape.transform(translation);
        }
        Some(HandleKind::Endpoint(idx)) => match &mut shape {
            Shape::Line(line) => {
                apply_polyline_endpoint(&mut line.start, &mut line.end, idx, delta)
            }
            Shape::Arrow(arrow) => {
                apply_polyline_endpoint(&mut arrow.start, &mut arrow.end, idx, delta)
            }
            _ => {}
        },
        Some(HandleKind::IntermediatePoint(idx)) => match &mut shape {
            Shape::Line(line) => {
                if let Some(pt) = line.intermediate_points.get_mut(idx) {
                    pt.x += delta.x;
                    pt.y += delta.y;
                }
            }
            Shape::Arrow(arrow) => {
                if let Some(pt) = arrow.intermediate_points.get_mut(idx) {
                    pt.x += delta.x;
                    pt.y += delta.y;
                }
            }
            _ => {}
        },
        Some(HandleKind::SegmentMidpoint(seg_idx)) => match &mut shape {
            Shape::Line(line) => {
                let pts = line.all_points();
                insert_segment_midpoint(&mut line.intermediate_points, &pts, seg_idx, delta);
            }
            Shape::Arrow(arrow) => {
                let pts = arrow.all_points();
                insert_segment_midpoint(&mut arrow.intermediate_points, &pts, seg_idx, delta);
            }
            _ => {}
        },
        Some(HandleKind::Corner(corner)) => match &mut shape {
            Shape::Rectangle(rect) => {
                apply_corner_resize_rect(rect, corner, delta, keep_aspect_ratio);
            }
            Shape::Diamond(diamond) => {
                apply_corner_resize_diamond(diamond, corner, delta, keep_aspect_ratio);
            }
            Shape::Ellipse(ellipse) => {
                apply_corner_resize_ellipse(ellipse, corner, delta, keep_aspect_ratio);
            }
            Shape::Freehand(freehand) => {
                apply_corner_resize_freehand(freehand, corner, delta, keep_aspect_ratio);
            }
            Shape::Image(image) => {
                apply_corner_resize_image(image, corner, delta, keep_aspect_ratio);
            }
            Shape::Group(group) => {
                apply_corner_resize_group(group, corner, delta, keep_aspect_ratio);
            }
            Shape::Text(text) => {
                apply_corner_resize_text(text, corner, delta, keep_aspect_ratio);
            }
            Shape::Math(math) => {
                apply_corner_resize_math(math, corner, delta, keep_aspect_ratio);
            }
            _ => {}
        },
        Some(HandleKind::Edge(_)) => {}
        Some(HandleKind::Rotate) => {}
    }

    shape
}

fn apply_polyline_endpoint(start: &mut Point, end: &mut Point, idx: usize, delta: kurbo::Vec2) {
    if idx == 0 {
        start.x += delta.x;
        start.y += delta.y;
        return;
    }
    end.x += delta.x;
    end.y += delta.y;
}

fn insert_segment_midpoint(
    intermediate_points: &mut Vec<Point>,
    all_pts: &[Point],
    seg_idx: usize,
    delta: kurbo::Vec2,
) {
    if seg_idx >= all_pts.len().saturating_sub(1) {
        return;
    }
    let mid = Point::new(
        (all_pts[seg_idx].x + all_pts[seg_idx + 1].x) / 2.0 + delta.x,
        (all_pts[seg_idx].y + all_pts[seg_idx + 1].y) / 2.0 + delta.y,
    );
    intermediate_points.insert(seg_idx, mid);
}
