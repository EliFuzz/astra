use super::super::super::types::Corner;
use super::resize_bounds;
use crate::shapes::ShapeTrait;
use kurbo::{Point, Vec2};

pub(crate) fn apply_corner_resize_text(
    text: &mut crate::shapes::Text,
    corner: Corner,
    delta: Vec2,
    _keep_aspect_ratio: bool,
) {
    let bounds = text.bounds();
    if bounds.height() < 0.001 {
        return;
    }
    let (x0, y0, _, y1) = resize_bounds(bounds, corner, delta);
    let scale = (y1 - y0).max(1.0) / bounds.height();
    text.font_size = (text.font_size * scale).max(4.0);
    text.position = Point::new(x0, y0);
    text.invalidate_cache();
}

pub(crate) fn apply_corner_resize_math(
    math: &mut crate::shapes::Math,
    corner: Corner,
    delta: Vec2,
    _keep_aspect_ratio: bool,
) {
    let bounds = math.bounds();
    if bounds.height() < 0.001 {
        return;
    }
    let (x0, y0, _, y1) = resize_bounds(bounds, corner, delta);
    let scale = (y1 - y0).max(1.0) / bounds.height();
    let new_font_size = (math.font_size * scale).max(4.0);
    let ascent = math
        .cached_size()
        .map(|(_, h, _)| h)
        .unwrap_or(math.font_size);
    let ascent_ratio = ascent / math.font_size.max(0.001);
    math.position = Point::new(x0, y0 + new_font_size * ascent_ratio);
    math.font_size = new_font_size;
    math.invalidate_cache();
}

pub(crate) fn apply_corner_resize_rect(
    rect: &mut crate::shapes::Rectangle,
    corner: Corner,
    delta: Vec2,
    keep_aspect_ratio: bool,
) {
    let (pos, w, h) = box_resize(rect.bounds(), corner, delta, keep_aspect_ratio);
    rect.position = pos;
    rect.width = w;
    rect.height = h;
}

pub(crate) fn apply_corner_resize_diamond(
    diamond: &mut crate::shapes::Diamond,
    corner: Corner,
    delta: Vec2,
    keep_aspect_ratio: bool,
) {
    let (pos, w, h) = box_resize(diamond.bounds(), corner, delta, keep_aspect_ratio);
    diamond.position = pos;
    diamond.width = w;
    diamond.height = h;
}

pub(crate) fn apply_corner_resize_ellipse(
    ellipse: &mut crate::shapes::Ellipse,
    corner: Corner,
    delta: Vec2,
    keep_aspect_ratio: bool,
) {
    let (pos, w, h) = box_resize(ellipse.bounds(), corner, delta, keep_aspect_ratio);
    ellipse.center = Point::new(pos.x + w / 2.0, pos.y + h / 2.0);
    ellipse.radius_x = w / 2.0;
    ellipse.radius_y = h / 2.0;
}

pub(crate) fn apply_corner_resize_image(
    image: &mut crate::shapes::Image,
    corner: Corner,
    delta: Vec2,
    keep_aspect_ratio: bool,
) {
    let (pos, w, h) = box_resize(image.bounds(), corner, delta, keep_aspect_ratio);
    image.position = pos;
    image.width = w;
    image.height = h;
}

fn box_resize(
    bounds: kurbo::Rect,
    corner: Corner,
    delta: Vec2,
    keep_aspect_ratio: bool,
) -> (Point, f64, f64) {
    let (x0, y0, x1, y1) = resize_bounds(bounds, corner, delta);
    let (width, height) = if keep_aspect_ratio {
        let aspect = bounds.width() / bounds.height().max(0.1);
        let size = (x1 - x0).max(1.0).max(y1 - y0);
        (size, size / aspect)
    } else {
        ((x1 - x0).max(1.0), (y1 - y0).max(1.0))
    };
    (Point::new(x0, y0), width, height)
}
