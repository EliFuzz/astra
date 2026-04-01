use super::super::EventHandler;
use crate::canvas::Canvas;
use crate::input::InputState;
use crate::snap::{GRID_SIZE, snap_line_endpoint_isometric, snap_to_grid};
use crate::tools::ToolKind;
use kurbo::Point;

impl EventHandler {
    pub fn handle_drag(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        input: &InputState,
        grid_snap_enabled: bool,
        smart_snap_enabled: bool,
        angle_snap_enabled: bool,
    ) {
        self.last_snap = None;
        self.last_angle_snap = None;
        self.smart_guides.clear();

        if self.manipulation.is_some() {
            self.drag_manipulation(
                canvas,
                world_point,
                input,
                grid_snap_enabled,
                smart_snap_enabled,
                angle_snap_enabled,
            );
            return;
        }

        if self.multi_move.is_some() {
            self.drag_multi_move(canvas, world_point, grid_snap_enabled, smart_snap_enabled);
            return;
        }

        if let Some(sel_rect) = &mut self.selection_rect {
            sel_rect.current = world_point;
            return;
        }

        if canvas.tool_manager.current_tool == ToolKind::Eraser && !self.eraser_points.is_empty() {
            self.eraser_points.push(world_point);
            self.apply_eraser(canvas);
            if let Some(last) = self.eraser_points.last().copied() {
                self.eraser_points.clear();
                self.eraser_points.push(last);
            }
            return;
        }

        if canvas.tool_manager.current_tool == ToolKind::LaserPointer {
            self.laser_position = Some(world_point);
            self.laser_trail.push_back((world_point, 1.0));
            if self.laser_trail.len() > super::super::handler::LASER_TRAIL_CAP {
                self.laser_trail.pop_front();
            }
            return;
        }

        if canvas.tool_manager.is_active() {
            let tool = canvas.tool_manager.current_tool;

            if matches!(tool, ToolKind::Line | ToolKind::Arrow) {
                if let Some(start) = self.line_start_point {
                    let angle_result = snap_line_endpoint_isometric(
                        start,
                        world_point,
                        angle_snap_enabled,
                        grid_snap_enabled,
                        false,
                        GRID_SIZE,
                    );

                    if angle_result.snapped {
                        self.last_angle_snap = Some(angle_result);
                    }

                    canvas.tool_manager.update(angle_result.point);
                    return;
                }
            }

            let point = if grid_snap_enabled
                && !matches!(tool, ToolKind::Freehand | ToolKind::Highlighter)
            {
                let snap_result = snap_to_grid(world_point, GRID_SIZE);
                self.last_snap = Some(snap_result);
                snap_result.point
            } else {
                world_point
            };
            canvas.tool_manager.update(point);
        }
    }
}
