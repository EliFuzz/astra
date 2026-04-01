use super::super::AppState;
use astra_canvas::shapes::Shape;
use astra_canvas::tools::ToolKind;
use kurbo::Point;
use winit::event::{ElementState, MouseButton};

fn handle_press(state: &mut AppState, mouse_btn: MouseButton, position: Point) {
    if mouse_btn == MouseButton::Left {
        let world_point = state.canvas.camera.screen_to_world(position);

        if let Some(text_id) = state.event_handler.editing_text {
            let hits = state
                .canvas
                .document
                .shapes_at_point(world_point, 5.0 / state.canvas.camera.zoom);
            let clicked_on_editing = hits.first().map(|&id| {
                if id == text_id { return true; }
                if let Some(Shape::Group(g)) = state.canvas.document.get_shape(id) {
                    return g.children.iter().any(|c| c.id() == text_id);
                }
                false
            }).unwrap_or(false);

            if clicked_on_editing {
                if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
                    let local_x = (world_point.x - text.position.x) as f32;
                    let local_y = (world_point.y - text.position.y) as f32;

                    if state.text_edit_state.is_none() {
                        let mut edit_state =
                            astra_render::TextEditState::new(&text.content, text.font_size as f32);
                        edit_state.cursor_reset();
                        state.text_edit_state = Some(edit_state);
                    }

                    if let Some(edit_state) = &mut state.text_edit_state {
                        let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();
                        if state.input.is_triple_click() {
                            let mut drv = edit_state.driver(font_cx, layout_cx);
                            drv.select_all();
                            edit_state.cursor_reset();
                        } else if state.input.is_double_click() && !text.content.is_empty() {
                            let mut drv = edit_state.driver(font_cx, layout_cx);
                            drv.select_word_at_point(local_x, local_y);
                            edit_state.cursor_reset();
                        } else {
                            edit_state.handle_mouse_down(
                                local_x,
                                local_y,
                                state.input.shift(),
                                font_cx,
                                layout_cx,
                            );
                        }
                    }
                }
            } else {
                state.event_handler.exit_text_edit(&mut state.canvas);
                state.text_edit_state = None;
                state.event_handler.handle_press(
                    &mut state.canvas,
                    world_point,
                    &state.input,
                    state.ui_state.grid_snap_enabled,
                );
            }
        } else {
            let was_double_click = state.input.is_double_click();
            state.event_handler.handle_press(
                &mut state.canvas,
                world_point,
                &state.input,
                state.ui_state.grid_snap_enabled,
            );

            if let Some(text_id) = state.event_handler.editing_text {
                if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
                    let mut edit_state =
                        astra_render::TextEditState::new(&text.content, text.font_size as f32);
                    if was_double_click && !text.content.is_empty() {
                        let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();
                        let mut drv = edit_state.driver(font_cx, layout_cx);
                        drv.select_all();
                        edit_state.cursor_reset();
                    } else {
                        edit_state.cursor_reset();
                        let (font_cx, layout_cx) = state.shape_renderer.contexts_mut();
                        let mut drv = edit_state.driver(font_cx, layout_cx);
                        drv.move_to_text_end();
                    }
                    state.text_edit_state = Some(edit_state);
                }
            }

            if let Some(math_id) = state.event_handler.pending_math_edit.take() {
                if let Some(Shape::Math(math)) = state.canvas.document.get_shape(math_id) {
                    state.ui_state.math_editor = Some((math_id, math.latex.clone()));
                }
            }
        }
    }
}

fn handle_release(state: &mut AppState, mouse_btn: MouseButton, position: Point) {
    if mouse_btn == MouseButton::Left {
        if let Some(edit_state) = &mut state.text_edit_state {
            edit_state.handle_mouse_up();
        }

        let world_point = state.canvas.camera.screen_to_world(position);
        let current_style = state.ui_state.to_shape_style();
        state.event_handler.handle_release(
            &mut state.canvas,
            world_point,
            &state.input,
            &current_style,
            state.ui_state.grid_snap_enabled,
            state.ui_state.angle_snap_enabled,
        );

        if matches!(
            state.canvas.tool_manager.current_tool,
            ToolKind::Rectangle
                | ToolKind::Diamond
                | ToolKind::Ellipse
                | ToolKind::Line
                | ToolKind::Arrow
                | ToolKind::Freehand
                | ToolKind::Highlighter
                | ToolKind::Text
        ) {
            state.canvas.set_tool(ToolKind::Select);
        }

        state.event_handler.clear_snap();

        if state.text_edit_state.is_none() {
            if let Some(text_id) = state.event_handler.editing_text {
                if let Some(Shape::Text(text)) = state.canvas.document.get_shape_deep(text_id) {
                    let mut edit_state =
                        astra_render::TextEditState::new(&text.content, text.font_size as f32);
                    edit_state.cursor_reset();
                    state.text_edit_state = Some(edit_state);
                }
            }
        }
    }
}

pub fn handle_mouse_input(
    state: &mut AppState,
    btn_state: ElementState,
    button: MouseButton,
    egui_wants_input: bool,
) {
    if egui_wants_input {
        state.needs_redraw = true;
        state.window.request_redraw();
        return;
    }

    let mouse_btn = match button {
        MouseButton::Left => MouseButton::Left,
        MouseButton::Right => MouseButton::Right,
        MouseButton::Middle => MouseButton::Middle,
        _ => return,
    };

    let position = state.input.mouse_position();

    match btn_state {
        ElementState::Pressed => handle_press(state, mouse_btn, position),
        ElementState::Released => handle_release(state, mouse_btn, position),
    }

    state.needs_redraw = true;
    state.window.request_redraw();
}
