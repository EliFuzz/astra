use super::binding::ArrowBinding;
use super::super::line::PathStyle;
use super::super::{ShapeId, ShapeStyle, StrokeStyle};
use kurbo::{Point, Vec2};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arrow {
    pub(crate) id: ShapeId,
    pub start: Point,
    pub end: Point,
    #[serde(default)]
    pub intermediate_points: Vec<Point>,
    #[serde(default)]
    pub path_style: PathStyle,
    #[serde(default)]
    pub stroke_style: StrokeStyle,
    pub head_size: f64,
    pub style: ShapeStyle,
    #[serde(default)]
    pub start_binding: Option<ArrowBinding>,
    #[serde(default)]
    pub end_binding: Option<ArrowBinding>,
}

impl Arrow {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            id: Uuid::new_v4(),
            start,
            end,
            intermediate_points: Vec::new(),
            path_style: PathStyle::Direct,
            stroke_style: StrokeStyle::default(),
            head_size: 15.0,
            style: ShapeStyle::default(),
            start_binding: None,
            end_binding: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: ShapeId,
        start: Point,
        end: Point,
        intermediate_points: Vec<Point>,
        path_style: PathStyle,
        stroke_style: StrokeStyle,
        head_size: f64,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            start,
            end,
            intermediate_points,
            path_style,
            stroke_style,
            head_size,
            style,
            start_binding: None,
            end_binding: None,
        }
    }

    pub fn from_points(points: Vec<Point>, path_style: PathStyle) -> Self {
        let start = points.first().copied().unwrap_or(Point::ZERO);
        let end = points.last().copied().unwrap_or(Point::ZERO);
        let intermediate_points = if points.len() > 2 {
            points[1..points.len() - 1].to_vec()
        } else {
            Vec::new()
        };
        Self {
            id: Uuid::new_v4(),
            start,
            end,
            intermediate_points,
            path_style,
            stroke_style: StrokeStyle::default(),
            head_size: 15.0,
            style: ShapeStyle::default(),
            start_binding: None,
            end_binding: None,
        }
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut pts = vec![self.start];
        pts.extend(&self.intermediate_points);
        pts.push(self.end);
        pts
    }

    pub fn direction(&self) -> Vec2 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        let len = (dx * dx + dy * dy).sqrt();
        if len < f64::EPSILON {
            Vec2::new(1.0, 0.0)
        } else {
            Vec2::new(dx / len, dy / len)
        }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}
