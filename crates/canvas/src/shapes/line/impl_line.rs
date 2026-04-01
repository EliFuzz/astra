use super::{Line, PathStyle};
use crate::shapes::{ShapeId, ShapeStyle, StrokeStyle};
use kurbo::{Line as KurboLine, Point};
use uuid::Uuid;

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self {
            id: Uuid::new_v4(),
            start,
            end,
            intermediate_points: Vec::new(),
            path_style: PathStyle::Direct,
            stroke_style: StrokeStyle::default(),
            closed: false,
            style: ShapeStyle::default(),
        }
    }

    pub fn reconstruct(
        id: ShapeId,
        start: Point,
        end: Point,
        intermediate_points: Vec<Point>,
        path_style: PathStyle,
        stroke_style: StrokeStyle,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            start,
            end,
            intermediate_points,
            path_style,
            stroke_style,
            closed: false,
            style,
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
            closed: false,
            style: ShapeStyle::default(),
        }
    }

    pub fn all_points(&self) -> Vec<Point> {
        let mut pts = vec![self.start];
        pts.extend(&self.intermediate_points);
        pts.push(self.end);
        pts
    }

    pub fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn midpoint(&self) -> Point {
        Point::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }

    pub fn as_kurbo(&self) -> KurboLine {
        KurboLine::new(self.start, self.end)
    }
}
