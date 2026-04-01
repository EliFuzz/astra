use kurbo::Point;
use serde::{Deserialize, Serialize};

pub const HANDLE_SIZE: f64 = 16.0;
pub const HANDLE_HIT_TOLERANCE: f64 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HandleKind {
    Endpoint(usize),
    IntermediatePoint(usize),
    SegmentMidpoint(usize),
    Corner(Corner),
    Edge(Edge),
    Rotate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub struct Handle {
    pub position: Point,
    pub kind: HandleKind,
}

impl Handle {
    pub fn new(position: Point, kind: HandleKind) -> Self {
        Self { position, kind }
    }

    pub fn hit_test(&self, point: Point, tolerance: f64) -> bool {
        let dx = point.x - self.position.x;
        let dy = point.y - self.position.y;
        let dist_sq = dx * dx + dy * dy;
        dist_sq <= tolerance * tolerance
    }
}
