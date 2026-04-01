use super::super::AppState;
use astra_canvas::shapes::Shape;
use astra_canvas::tools::ToolKind;
use astra_render::{AngleSnapInfo, RenderContext, Renderer, TextEditState};
use kurbo::{Point, Size};

pub fn build_render_scene(state: &mut AppState) -> vello::Scene {
    let viewport_size = Size::new(
        state.canvas.viewport_size.width,
        state.canvas.viewport_size.height,
    );
    let selection_rect = state.event_handler.selection_rect().map(|sr| sr.to_rect());
    let snap_point = state.event_handler.last_snap.as_ref().map(|s| s.point);

    let angle_snap_info = state
        .event_handler
        .last_angle_snap
        .as_ref()
        .filter(|_| state.ui_state.angle_snap_enabled)
        .map(|angle_snap| AngleSnapInfo {
            start_point: state
                .event_handler
                .line_start_point
                .unwrap_or(kurbo::Point::ZERO),
            end_point: angle_snap.point,
            angle_degrees: angle_snap.angle_degrees,
            is_snapped: angle_snap.snapped,
        });

    let rotation_info =
        state
            .event_handler
            .rotation_state
            .as_ref()
            .map(|rs| astra_render::RotationInfo {
                center: rs.center,
                angle: rs.angle,
                snapped: rs.snapped,
            });

    let eraser_cursor = if state.canvas.tool_manager.current_tool == ToolKind::Eraser {
        state
            .event_handler
            .eraser_path()
            .last()
            .map(|p| (*p, state.event_handler.eraser_radius))
    } else {
        None
    };

    let laser_pointer = if state.canvas.tool_manager.current_tool == ToolKind::LaserPointer {
        state.event_handler.laser_position.map(|pos| {
            let trail: Vec<(kurbo::Point, f64)> =
                state.event_handler.laser_trail.iter().copied().collect();
            (pos, trail)
        })
    } else {
        None
    };

    let smart_guides = state.event_handler.smart_guides.clone();

    let arrow_snap_targets = compute_arrow_snap_targets(state);

    let render_ctx = RenderContext::new(&state.canvas, viewport_size)
        .with_scale_factor(state.window.scale_factor())
        .with_background(state.config.background_color)
        .with_grid(state.config.grid_style)
        .with_selection_rect(selection_rect)
        .with_editing_shape(state.event_handler.editing_text)
        .with_snap_point(snap_point)
        .with_angle_snap(angle_snap_info)
        .with_rotation_info(rotation_info)
        .with_smart_guides(smart_guides)
        .with_eraser_cursor(eraser_cursor)
        .with_laser_pointer(laser_pointer)
        .with_arrow_snap_targets(arrow_snap_targets);

    state.shape_renderer.build_scene(&render_ctx);

    if let Some(text_id) = state.event_handler.editing_text {
        if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
            let camera_transform = state.canvas.camera.transform();

            if state.text_edit_state.is_none() {
                let mut edit_state = TextEditState::new(&text.content, text.font_size as f32);
                edit_state.cursor_reset();
                state.text_edit_state = Some(edit_state);
            }

            if let Some(edit_state) = &mut state.text_edit_state {
                edit_state.cursor_blink();
                state.shape_renderer.render_text_editing(
                    text,
                    edit_state,
                    camera_transform,
                    state.event_handler.text_edit_anchor,
                );
            }
        }
    }

    state.shape_renderer.take_scene()
}

fn compute_arrow_snap_targets(state: &AppState) -> Vec<Point> {
    use astra_canvas::selection::HandleKind;
    use astra_canvas::snap::ARROW_BIND_MIDPOINT_RADIUS;
    use astra_canvas::tools::{ToolKind, ToolState};

    let is_arrow_tool_active = state.canvas.tool_manager.current_tool == ToolKind::Arrow
        && matches!(state.canvas.tool_manager.state, ToolState::Active { .. });

    let is_arrow_endpoint_drag = state
        .event_handler
        .manipulation
        .as_ref()
        .map(|m| {
            matches!(m.original_shape, Shape::Arrow(_))
                && matches!(m.handle, Some(HandleKind::Endpoint(_)))
        })
        .unwrap_or(false);

    if !is_arrow_tool_active && !is_arrow_endpoint_drag {
        return Vec::new();
    }

    let cursor = match state.event_handler.manipulation.as_ref() {
        Some(m) => m.current_point,
        None => match &state.canvas.tool_manager.state {
            ToolState::Active { current, .. } => *current,
            _ => return Vec::new(),
        },
    };

    let detection_r = ARROW_BIND_MIDPOINT_RADIUS + 40.0;
    let viewport = state.canvas.visible_world_bounds().inflate(100.0, 100.0);

    state
        .canvas
        .document
        .shapes_ordered()
        .filter(|s| {
            matches!(
                s,
                Shape::Rectangle(_)
                    | Shape::Ellipse(_)
                    | Shape::Text(_)
                    | Shape::Math(_)
                    | Shape::Image(_)
            )
        })
        .filter(|s| {
            !viewport
                .intersect(s.bounds().inflate(1.0, 1.0))
                .is_zero_area()
        })
        .filter(|s| {
            let b = s.bounds();
            let dx = (cursor.x - (b.x0 + b.x1) / 2.0).abs() - b.width() / 2.0;
            let dy = (cursor.y - (b.y0 + b.y1) / 2.0).abs() - b.height() / 2.0;
            dx.max(0.0).hypot(dy.max(0.0)) <= detection_r
        })
        .flat_map(|s| {
            let b = s.bounds();
            let cx = (b.x0 + b.x1) / 2.0;
            let cy = (b.y0 + b.y1) / 2.0;
            [
                Point::new(cx, b.y0),
                Point::new(b.x1, cy),
                Point::new(cx, b.y1),
                Point::new(b.x0, cy),
            ]
        })
        .collect()
}
