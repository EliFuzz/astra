use kurbo::{Point, Rect};

#[derive(Debug, Clone)]
pub struct SelectionRect {
    pub start: Point,
    pub current: Point,
}

impl SelectionRect {
    pub fn to_rect(&self) -> Rect {
        Rect::new(
            self.start.x.min(self.current.x),
            self.start.y.min(self.current.y),
            self.start.x.max(self.current.x),
            self.start.y.max(self.current.y),
        )
    }
}

#[derive(Debug, Clone)]
pub struct RotationState {
    pub center: Point,
    pub angle: f64,
    pub snapped: bool,
}
