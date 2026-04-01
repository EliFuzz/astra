use super::super::{ToolKind, ToolState};
use super::ToolManager;
use crate::shapes::{Arrow, Diamond, Ellipse, Freehand, Line, Rectangle, Shape, Text};
use kurbo::Point;

pub(super) fn generate_tool_seed() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};

    static SEED_COUNTER: AtomicU32 = AtomicU32::new(1);

    let counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut x = counter.wrapping_mul(0x9E3779B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EBCA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2AE35);
    x ^= x >> 16;
    x
}

impl ToolManager {
    pub fn preview_shape(&self) -> Option<Shape> {
        if let ToolState::Active {
            start,
            current,
            seed,
            ..
        } = &self.state
        {
            if self.current_tool == ToolKind::Freehand || self.current_tool == ToolKind::Highlighter
            {
                return self.create_freehand_preview(*seed);
            }
            self.create_shape_with_seed(*start, *current, *seed)
        } else {
            None
        }
    }

    pub(super) fn create_freehand_preview(&self, seed: u32) -> Option<Shape> {
        if self.freehand_points.len() >= 2 {
            let mut freehand = Freehand::from_points_with_pressure(
                self.freehand_points.clone(),
                self.freehand_pressures.clone(),
            );
            freehand.style = self.current_style.clone();
            freehand.style.seed = seed;

            if self.current_tool == ToolKind::Highlighter {
                freehand.style.stroke_width = self.current_style.stroke_width.max(12.0);
                freehand.style.stroke_color.a = 128;
            }

            Some(Shape::Freehand(freehand))
        } else {
            None
        }
    }

    pub(super) fn create_shape_with_seed(
        &self,
        start: Point,
        end: Point,
        seed: u32,
    ) -> Option<Shape> {
        let mut shape = match self.current_tool {
            ToolKind::Rectangle => {
                let mut rect = Rectangle::from_corners(start, end);
                rect.corner_radius = self.corner_radius;
                Some(Shape::Rectangle(rect))
            }
            ToolKind::Diamond => Some(Shape::Diamond(Diamond::from_corners(start, end))),
            ToolKind::Ellipse => {
                let rect = kurbo::Rect::new(
                    start.x.min(end.x),
                    start.y.min(end.y),
                    start.x.max(end.x),
                    start.y.max(end.y),
                );
                Some(Shape::Ellipse(Ellipse::from_rect(rect)))
            }
            ToolKind::Line => Some(Shape::Line(Line::new(start, end))),
            ToolKind::Arrow => Some(Shape::Arrow(Arrow::new(start, end))),
            ToolKind::Freehand | ToolKind::Highlighter => {
                return self.create_freehand_preview(seed);
            }
            ToolKind::Text => Some(Shape::Text(Text::new(start, String::new()))),
            ToolKind::Select
            | ToolKind::Pan
            | ToolKind::Eraser
            | ToolKind::LaserPointer
            | ToolKind::InsertImage
            | ToolKind::Math => None,
        };

        if let Some(ref mut s) = shape {
            let mut style = self.current_style.clone();
            style.seed = seed;
            *s.style_mut() = style;
        }

        shape
    }
}
