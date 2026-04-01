use super::super::AppState;
use astra_canvas::tools::ToolKind;
use kurbo::Vec2;
use winit::event::{MouseScrollDelta, Touch, TouchPhase};

pub fn handle_mouse_wheel(state: &mut AppState, delta: MouseScrollDelta, egui_wants_input: bool) {
    if egui_wants_input {
        return;
    }

    let scroll = match delta {
        MouseScrollDelta::LineDelta(x, y) => Vec2::new(x as f64 * 20.0, y as f64 * 20.0),
        MouseScrollDelta::PixelDelta(pos) => Vec2::new(pos.x, pos.y),
    };

    let position = state.input.mouse_position();

    if state.input.ctrl() {
        let zoom_factor = match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                if y > 0.0 {
                    1.1
                } else {
                    0.9
                }
            }
            MouseScrollDelta::PixelDelta(pos) => (1.0 + pos.y * 0.004).clamp(0.85, 1.15),
        };
        state.canvas.camera.zoom_at(position, zoom_factor);
    } else {
        state.canvas.camera.pan(scroll);
    }
    state.needs_redraw = true;
    state.window.request_redraw();
}

pub fn handle_pinch_gesture(state: &mut AppState, delta: f64, egui_wants_input: bool) {
    if egui_wants_input {
        return;
    }
    if delta.is_nan() || delta == 0.0 {
        return;
    }
    let zoom_factor = 1.0 + delta;
    let position = state.input.mouse_position();
    state.canvas.camera.zoom_at(position, zoom_factor);
    state.needs_redraw = true;
    state.window.request_redraw();
}

pub fn handle_touch(state: &mut AppState, touch: Touch, egui_wants_input: bool) {
    use kurbo::Point;

    if egui_wants_input {
        return;
    }

    let pos = Point::new(touch.location.x, touch.location.y);
    let world_point = state.canvas.camera.screen_to_world(pos);

    let prev_pos = state.input.primary_touch();

    let gesture = state.input.process_touch(&touch);
    let touch_count = state.input.touch_count();

    if let Some((pan_delta, zoom_delta, zoom_center)) = gesture {
        if (zoom_delta - 1.0).abs() > 0.001 {
            state.canvas.camera.zoom_at(zoom_center, zoom_delta);
        }
        if pan_delta.length() > 0.1 {
            state.canvas.camera.pan(pan_delta);
        }
    } else if touch_count <= 1 {
        let is_pan_tool = state.canvas.tool_manager.current_tool == ToolKind::Pan;

        if is_pan_tool {
            if touch.phase == TouchPhase::Moved {
                if let Some(prev) = prev_pos {
                    let delta = Vec2::new(pos.x - prev.x, pos.y - prev.y);
                    state.canvas.camera.pan(delta);
                }
            }
        } else {
            match touch.phase {
                TouchPhase::Started => {
                    state.event_handler.handle_press(
                        &mut state.canvas,
                        world_point,
                        &state.input,
                        state.ui_state.grid_snap_enabled,
                    );
                }
                TouchPhase::Moved => {
                    state.event_handler.handle_drag(
                        &mut state.canvas,
                        world_point,
                        &state.input,
                        state.ui_state.grid_snap_enabled,
                        state.ui_state.smart_snap_enabled,
                        state.ui_state.angle_snap_enabled,
                    );
                }
                TouchPhase::Ended => {
                    let current_style = state.ui_state.to_shape_style();
                    state.event_handler.handle_release(
                        &mut state.canvas,
                        world_point,
                        &state.input,
                        &current_style,
                        state.ui_state.grid_snap_enabled,
                        state.ui_state.angle_snap_enabled,
                    );
                }
                TouchPhase::Cancelled => {
                    state.event_handler.cancel(&mut state.canvas);
                }
            }
        }
    }
    state.needs_redraw = true;
    state.window.request_redraw();
}
