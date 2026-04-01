use kurbo::Rect;

pub(super) fn convert_rect(rect: &parley::BoundingBox) -> Rect {
    Rect::new(rect.x0, rect.y0, rect.x1, rect.y1)
}
