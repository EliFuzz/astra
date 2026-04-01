use kurbo::Point;

use crate::canvas::Canvas;
use crate::input::InputState;
use crate::shapes::{Freehand, Shape, ShapeStyle, ShapeTrait, Text};
use crate::snap::{GRID_SIZE, snap_line_endpoint_isometric, snap_to_grid};
use crate::tools::ToolKind;

use crate::event_handler::EventHandler;

use super::super::snap_helpers::{
    collect_routing_obstacles, find_shape_binding_snap, make_arrow_binding,
};

impl EventHandler {
    pub(super) fn release_tools(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        _input: &InputState,
        current_style: &ShapeStyle,
        grid_snap_enabled: bool,
        angle_snap_enabled: bool,
    ) {
        match canvas.tool_manager.current_tool {
            ToolKind::Select | ToolKind::Pan => {}
            ToolKind::Freehand => {
                let points = canvas.tool_manager.freehand_points();
                let pressures = canvas.tool_manager.freehand_pressures();
                if points.len() >= 2 {
                    let mut freehand =
                        Freehand::from_points_with_pressure(points.to_vec(), pressures.to_vec());
                    freehand.simplify(2.0);
                    freehand.style = current_style.clone();
                    let shape_id = freehand.id();
                    canvas.document.push_undo();
                    canvas.document.add_shape(Shape::Freehand(freehand));
                    canvas.clear_selection();
                    canvas.add_to_selection(shape_id);
                }
                canvas.tool_manager.cancel();
            }
            ToolKind::Highlighter => {
                let points = canvas.tool_manager.freehand_points();
                let pressures = canvas.tool_manager.freehand_pressures();
                if points.len() >= 2 {
                    let mut freehand =
                        Freehand::from_points_with_pressure(points.to_vec(), pressures.to_vec());
                    freehand.simplify(2.0);
                    freehand.style = current_style.clone();
                    freehand.style.stroke_width = current_style.stroke_width.max(12.0);
                    freehand.style.stroke_color.a = 128;
                    let shape_id = freehand.id();
                    canvas.document.push_undo();
                    canvas.document.add_shape(Shape::Freehand(freehand));
                    canvas.clear_selection();
                    canvas.add_to_selection(shape_id);
                }
                canvas.tool_manager.cancel();
            }
            ToolKind::Eraser => {
                self.eraser_points.clear();
            }
            ToolKind::LaserPointer => {
                self.laser_trail.clear();
                self.laser_position = None;
            }
            ToolKind::InsertImage => {}
            ToolKind::Text => {
                if self.editing_text.is_some() {
                    return;
                }

                let mut text = Text::new(world_point, String::new());
                text.style = current_style.clone();
                let shape = Shape::Text(text);
                let shape_id = shape.id();
                canvas.document.push_undo();
                canvas.document.add_shape(shape);
                canvas.clear_selection();
                canvas.add_to_selection(shape_id);
                self.enter_text_edit(canvas, shape_id);
                canvas.tool_manager.cancel();
            }
            ToolKind::Line | ToolKind::Arrow => {
                let end_point = if let Some(start) = self.line_start_point {
                    let angle_result = snap_line_endpoint_isometric(
                        start,
                        world_point,
                        angle_snap_enabled,
                        grid_snap_enabled,
                        false,
                        GRID_SIZE,
                    );
                    angle_result.point
                } else if grid_snap_enabled {
                    snap_to_grid(world_point, GRID_SIZE).point
                } else {
                    world_point
                };

                self.line_start_point = None;

                if let Some(mut shape) = canvas.tool_manager.end(end_point) {
                    let bounds = shape.bounds();
                    if bounds.width() > 1.0 || bounds.height() > 1.0 {
                        *shape.style_mut() = current_style.clone();

                        if let Shape::Arrow(arrow) = &mut shape {
                            let start_hit = find_shape_binding_snap(canvas, arrow.start, &[]);
                            let end_hit = find_shape_binding_snap(canvas, arrow.end, &[]);
                            if let Some((tid, pt, side, focus)) = start_hit {
                                arrow.start = pt;
                                arrow.start_binding = Some(make_arrow_binding(tid, side, focus));
                            }
                            if let Some((tid, pt, side, focus)) = end_hit {
                                arrow.end = pt;
                                arrow.end_binding = Some(make_arrow_binding(tid, side, focus));
                            }
                            if arrow.start_binding.is_some() || arrow.end_binding.is_some() {
                                let exit_side = arrow.start_binding.as_ref().map(|b| b.side);
                                let entry_side = arrow.end_binding.as_ref().map(|b| b.side);
                                let mut exclude = Vec::new();
                                if let Some(b) = &arrow.start_binding {
                                    exclude.push(b.target_id);
                                }
                                if let Some(b) = &arrow.end_binding {
                                    exclude.push(b.target_id);
                                }
                                let obstacles = collect_routing_obstacles(canvas, &exclude);
                                arrow.intermediate_points = crate::elbow::compute_routed_path(
                                    arrow.start,
                                    exit_side,
                                    arrow.end,
                                    entry_side,
                                    &obstacles,
                                );
                            }
                        }

                        let shape_id = shape.id();
                        canvas.document.push_undo();
                        canvas.document.add_shape(shape);
                        canvas.clear_selection();
                        canvas.add_to_selection(shape_id);
                    }
                }
            }
            _ => {
                let end_point = if grid_snap_enabled {
                    snap_to_grid(world_point, GRID_SIZE).point
                } else {
                    world_point
                };

                if let Some(mut shape) = canvas.tool_manager.end(end_point) {
                    let bounds = shape.bounds();
                    if bounds.width() > 1.0 || bounds.height() > 1.0 {
                        *shape.style_mut() = current_style.clone();
                        let shape_id = shape.id();
                        canvas.document.push_undo();
                        canvas.document.add_shape(shape);
                        canvas.clear_selection();
                        canvas.add_to_selection(shape_id);
                    }
                }
            }
        }
    }
}
