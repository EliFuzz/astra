use super::super::{ToolKind, ToolState};
use super::tool_manager::ToolManager;
use crate::shapes::ShapeStyle;
use kurbo::Point;

impl Default for ToolManager {
    fn default() -> Self {
        Self {
            current_tool: ToolKind::default(),
            state: ToolState::default(),
            freehand_points: Vec::new(),
            freehand_pressures: Vec::new(),
            last_point_time: None,
            last_point_pos: None,
            smoothed_pressure: 1.0,
            current_style: ShapeStyle::default(),
            corner_radius: 0.0,
            calligraphy_mode: false,
            pressure_simulation: false,
            msd_pos: Point::ZERO,
            msd_vel: Point::ZERO,
        }
    }
}
