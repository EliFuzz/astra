use crate::platform::Instant;
use super::super::{ToolKind, ToolState};
use crate::shapes::ShapeStyle;
use kurbo::Point;

#[derive(Debug, Clone)]
pub struct ToolManager {
    pub current_tool: ToolKind,
    pub state: ToolState,
    pub(super) freehand_points: Vec<Point>,
    pub(super) freehand_pressures: Vec<f64>,
    pub(super) last_point_time: Option<Instant>,
    pub(super) last_point_pos: Option<Point>,
    pub(super) smoothed_pressure: f64,
    pub current_style: ShapeStyle,
    pub corner_radius: f64,
    pub calligraphy_mode: bool,
    pub pressure_simulation: bool,
    pub(super) msd_pos: Point,
    pub(super) msd_vel: Point,
}
