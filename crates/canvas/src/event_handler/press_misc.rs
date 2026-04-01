use crate::canvas::Canvas;
use crate::snap::{GRID_SIZE, snap_to_grid};
use crate::tools::ToolKind;
use kurbo::Point;

use super::EventHandler;

pub(super) fn handle(
    handler: &mut EventHandler,
    canvas: &mut Canvas,
    world_point: Point,
    grid_snap_enabled: bool,
    tool: ToolKind,
) {
    match tool {
        ToolKind::Pan => {}
        ToolKind::Freehand | ToolKind::Highlighter => {
            canvas.tool_manager.begin(world_point);
        }
        ToolKind::Eraser => {
            handler.eraser_points.clear();
            handler.eraser_points.push(world_point);
        }
        ToolKind::LaserPointer => {
            handler.laser_position = Some(world_point);
            handler.laser_trail.push_back((world_point, 1.0));
        }
        ToolKind::InsertImage => {
            handler.pending_image_insert = Some(world_point);
        }
        ToolKind::Line | ToolKind::Arrow => {
            let start_point = if grid_snap_enabled {
                let snap_result = snap_to_grid(world_point, GRID_SIZE);
                handler.last_snap = Some(snap_result);
                snap_result.point
            } else {
                world_point
            };
            handler.line_start_point = Some(start_point);
            canvas.tool_manager.begin(start_point);
        }
        _ => {
            let start_point = if grid_snap_enabled {
                let snap_result = snap_to_grid(world_point, GRID_SIZE);
                handler.last_snap = Some(snap_result);
                snap_result.point
            } else {
                world_point
            };
            canvas.tool_manager.begin(start_point);
        }
    }
}
