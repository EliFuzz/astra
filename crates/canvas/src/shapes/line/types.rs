use crate::shapes::{ShapeId, ShapeStyle, StrokeStyle};
use kurbo::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathStyle {
    #[default]
    Direct,
    Flowing,
    Angular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub(crate) id: ShapeId,
    pub start: Point,
    pub end: Point,
    #[serde(default)]
    pub intermediate_points: Vec<Point>,
    #[serde(default)]
    pub path_style: PathStyle,
    #[serde(default)]
    pub stroke_style: StrokeStyle,
    #[serde(default)]
    pub closed: bool,
    pub style: ShapeStyle,
}
