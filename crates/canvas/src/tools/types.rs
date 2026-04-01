use crate::shapes::Shape;
use kurbo::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ToolKind {
    #[default]
    Select,
    Pan,
    Rectangle,
    Diamond,
    Ellipse,
    Line,
    Arrow,
    Freehand,
    Highlighter,
    Eraser,
    Text,
    Math,
    LaserPointer,
    InsertImage,
}

#[derive(Debug, Clone, Default)]
pub enum ToolState {
    #[default]
    Idle,
    Active {
        start: Point,
        current: Point,
        preview: Box<Option<Shape>>,
        seed: u32,
    },
}
