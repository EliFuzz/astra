use super::super::super::types::Corner;
use super::resize_bounds;
use crate::shapes::ShapeTrait;
use kurbo::Affine;

pub(crate) fn apply_corner_resize_freehand(
    freehand: &mut crate::shapes::Freehand,
    corner: Corner,
    delta: kurbo::Vec2,
    keep_aspect_ratio: bool,
) {
    if freehand.points.is_empty() {
        return;
    }

    let bounds = freehand.bounds();
    let (x0, y0, x1, y1) = resize_bounds(bounds, corner, delta);

    let old_width = bounds.width().max(1.0);
    let old_height = bounds.height().max(1.0);

    let (scale_x, scale_y) = if keep_aspect_ratio {
        let new_width = (x1 - x0).max(1.0);
        let new_height = (y1 - y0).max(1.0);
        let scale = (new_width / old_width).max(new_height / old_height);
        (scale, scale)
    } else {
        (
            (x1 - x0).max(1.0) / old_width,
            (y1 - y0).max(1.0) / old_height,
        )
    };

    for point in &mut freehand.points {
        let rel_x = point.x - bounds.x0;
        let rel_y = point.y - bounds.y0;
        point.x = x0 + rel_x * scale_x;
        point.y = y0 + rel_y * scale_y;
    }
}

pub(crate) fn apply_corner_resize_group(
    group: &mut crate::shapes::Group,
    corner: Corner,
    delta: kurbo::Vec2,
    keep_aspect_ratio: bool,
) {
    use crate::shapes::Shape;

    let bounds = group.bounds();
    if bounds.width() < 1.0 && bounds.height() < 1.0 {
        return;
    }

    let (x0, y0, x1, y1) = resize_bounds(bounds, corner, delta);

    let old_w = bounds.width().max(1.0);
    let old_h = bounds.height().max(1.0);

    let (sx, sy) = if keep_aspect_ratio {
        let s = ((x1 - x0).max(1.0) / old_w).max((y1 - y0).max(1.0) / old_h);
        (s, s)
    } else {
        ((x1 - x0).max(1.0) / old_w, (y1 - y0).max(1.0) / old_h)
    };

    let affine = Affine::translate(kurbo::Vec2::new(x0, y0))
        * Affine::scale_non_uniform(sx, sy)
        * Affine::translate(kurbo::Vec2::new(-bounds.x0, -bounds.y0));

    let has_text = group.children().iter().any(|c| matches!(c, Shape::Text(_)));

    for child in group.children_mut() {
        if has_text && matches!(child, Shape::Text(_)) {
            continue;
        }
        child.transform(affine);
    }

    if has_text {
        let scale = (sx + sy) / 2.0;
        for child in group.children_mut() {
            if let Shape::Text(text) = child {
                text.font_size *= scale;
                text.invalidate_cache();
            }
        }
        group.fit_and_center_text_children();
    }
}
