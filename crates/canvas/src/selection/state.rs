use super::handles::ROTATE_HANDLE_OFFSET;
use super::types::{Corner, Edge, HandleKind};
use crate::shapes::{ArrowBinding, Shape, ShapeId};
use kurbo::Point;

#[derive(Debug, Clone)]
pub struct ManipulationState {
    pub shape_id: ShapeId,
    pub handle: Option<HandleKind>,
    pub start_point: Point,
    pub current_point: Point,
    pub original_shape: Shape,
    pub pending_start_binding: Option<Option<ArrowBinding>>,
    pub pending_end_binding: Option<Option<ArrowBinding>>,
}

#[derive(Debug, Clone)]
pub struct MultiMoveState {
    pub start_point: Point,
    pub current_point: Point,
    pub original_shapes: std::collections::HashMap<ShapeId, Shape>,
    pub is_duplicate: bool,
    pub duplicated_ids: Vec<ShapeId>,
}

impl ManipulationState {
    pub fn new(
        shape_id: ShapeId,
        handle: Option<HandleKind>,
        start_point: Point,
        original_shape: Shape,
    ) -> Self {
        Self {
            shape_id,
            handle,
            start_point,
            current_point: start_point,
            original_shape,
            pending_start_binding: None,
            pending_end_binding: None,
        }
    }

    pub fn delta(&self) -> kurbo::Vec2 {
        kurbo::Vec2::new(
            self.current_point.x - self.start_point.x,
            self.current_point.y - self.start_point.y,
        )
    }
}

impl MultiMoveState {
    pub fn new(
        start_point: Point,
        original_shapes: std::collections::HashMap<ShapeId, Shape>,
    ) -> Self {
        Self {
            start_point,
            current_point: start_point,
            original_shapes,
            is_duplicate: false,
            duplicated_ids: Vec::new(),
        }
    }

    pub fn new_duplicate(
        start_point: Point,
        original_shapes: std::collections::HashMap<ShapeId, Shape>,
    ) -> Self {
        Self {
            start_point,
            current_point: start_point,
            original_shapes,
            is_duplicate: true,
            duplicated_ids: Vec::new(),
        }
    }

    pub fn delta(&self) -> kurbo::Vec2 {
        kurbo::Vec2::new(
            self.current_point.x - self.start_point.x,
            self.current_point.y - self.start_point.y,
        )
    }

    pub fn shape_ids(&self) -> Vec<ShapeId> {
        self.original_shapes.keys().copied().collect()
    }
}

pub fn get_manipulation_target_position(shape: &Shape, handle: Option<HandleKind>) -> Point {
    match handle {
        None => {
            let bounds = shape.bounds();
            Point::new(bounds.x0, bounds.y0)
        }
        Some(HandleKind::Endpoint(idx)) => match shape {
            Shape::Line(line) => {
                if idx == 0 {
                    line.start
                } else {
                    line.end
                }
            }
            Shape::Arrow(arrow) => {
                if idx == 0 {
                    arrow.start
                } else {
                    arrow.end
                }
            }
            _ => shape.bounds().center(),
        },
        Some(HandleKind::IntermediatePoint(idx)) => match shape {
            Shape::Line(line) => line
                .intermediate_points
                .get(idx)
                .copied()
                .unwrap_or(line.start),
            Shape::Arrow(arrow) => arrow
                .intermediate_points
                .get(idx)
                .copied()
                .unwrap_or(arrow.start),
            _ => shape.bounds().center(),
        },
        Some(HandleKind::SegmentMidpoint(seg_idx)) => match shape {
            Shape::Line(line) => segment_midpoint(&line.all_points(), seg_idx, line.start),
            Shape::Arrow(arrow) => segment_midpoint(&arrow.all_points(), seg_idx, arrow.start),
            _ => shape.bounds().center(),
        },
        Some(HandleKind::Corner(corner)) => {
            let bounds = shape.bounds();
            match corner {
                Corner::TopLeft => Point::new(bounds.x0, bounds.y0),
                Corner::TopRight => Point::new(bounds.x1, bounds.y0),
                Corner::BottomLeft => Point::new(bounds.x0, bounds.y1),
                Corner::BottomRight => Point::new(bounds.x1, bounds.y1),
            }
        }
        Some(HandleKind::Edge(edge)) => {
            let bounds = shape.bounds();
            match edge {
                Edge::Top => Point::new(bounds.center().x, bounds.y0),
                Edge::Right => Point::new(bounds.x1, bounds.center().y),
                Edge::Bottom => Point::new(bounds.center().x, bounds.y1),
                Edge::Left => Point::new(bounds.x0, bounds.center().y),
            }
        }
        Some(HandleKind::Rotate) => {
            let bounds = shape.bounds();
            let center = bounds.center();
            let rotation = shape.rotation();
            let half_h = bounds.height() / 2.0;
            let cos_r = rotation.cos();
            let sin_r = rotation.sin();
            Point::new(
                center.x - (half_h + ROTATE_HANDLE_OFFSET) * sin_r,
                center.y - (half_h + ROTATE_HANDLE_OFFSET) * cos_r,
            )
        }
    }
}

fn segment_midpoint(pts: &[Point], seg_idx: usize, default: Point) -> Point {
    if seg_idx < pts.len() - 1 {
        Point::new(
            (pts[seg_idx].x + pts[seg_idx + 1].x) / 2.0,
            (pts[seg_idx].y + pts[seg_idx + 1].y) / 2.0,
        )
    } else {
        default
    }
}
