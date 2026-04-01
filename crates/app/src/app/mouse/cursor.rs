use super::super::AppState;
use astra_canvas::shapes::Shape;
use astra_canvas::tools::ToolKind;
use kurbo::Point;
use winit::dpi::PhysicalPosition;
use winit::event::MouseButton;
use winit::window::CursorIcon;

pub fn handle_cursor_moved(
    state: &mut AppState,
    position: PhysicalPosition<f64>,
    egui_wants_input: bool,
) {
    let point = Point::new(position.x, position.y);

    if egui_wants_input {
        state.window.set_cursor(CursorIcon::Default);
        state.needs_redraw = true;
        state.window.request_redraw();
        return;
    }

    let world_point = state.canvas.camera.screen_to_world(point);

    if !state.input.is_button_pressed(MouseButton::Left) {
        use astra_canvas::selection::{Corner, HandleKind};
        let cursor = match state
            .event_handler
            .get_cursor_for_position(&state.canvas, world_point)
        {
            Some(Some(HandleKind::Corner(Corner::TopLeft | Corner::BottomRight))) => {
                CursorIcon::NwseResize
            }
            Some(Some(HandleKind::Corner(Corner::TopRight | Corner::BottomLeft))) => {
                CursorIcon::NeswResize
            }
            Some(Some(HandleKind::Edge(_))) => CursorIcon::EwResize,
            Some(Some(
                HandleKind::Endpoint(_)
                | HandleKind::IntermediatePoint(_)
                | HandleKind::SegmentMidpoint(_),
            )) => CursorIcon::Crosshair,
            Some(Some(HandleKind::Rotate)) => CursorIcon::Grab,
            Some(None) => CursorIcon::Move,
            None => CursorIcon::Default,
        };
        state.window.set_cursor(cursor);
    }

    if state.canvas.tool_manager.current_tool == ToolKind::LaserPointer {
        state.event_handler.laser_position = Some(world_point);
        state.event_handler.laser_trail.push_back((world_point, 1.0));
        if state.event_handler.laser_trail.len() > astra_canvas::event_handler::LASER_TRAIL_CAP {
            state.event_handler.laser_trail.pop_front();
        }
    }

    if state.input.is_button_pressed(MouseButton::Left) {
        if let Some(text_id) = state.event_handler.editing_text {
            if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
                let local_x = (world_point.x - text.position.x) as f32;
                let local_y = (world_point.y - text.position.y) as f32;

                if let Some(edit_state) = &mut state.text_edit_state {
                    let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();
                    edit_state.handle_mouse_drag(local_x, local_y, font_cx, layout_cx);
                }
            }
        } else if state.event_handler.is_manipulating()
            || state.event_handler.is_selecting()
            || state.canvas.tool_manager.current_tool == ToolKind::Eraser
            || state.canvas.tool_manager.is_active()
        {
            state.event_handler.handle_drag(
                &mut state.canvas,
                world_point,
                &state.input,
                state.ui_state.grid_snap_enabled,
                state.ui_state.smart_snap_enabled,
                state.ui_state.angle_snap_enabled,
            );
        } else if state.canvas.tool_manager.current_tool == ToolKind::Pan {
            let delta = state.input.cursor_diff();
            state.canvas.camera.pan(delta);
        }
    }

    if state.input.is_button_pressed(MouseButton::Middle) {
        let delta = state.input.cursor_diff();
        state.canvas.camera.pan(delta);
    }

    state.needs_redraw = true;
    state.window.request_redraw();
}
