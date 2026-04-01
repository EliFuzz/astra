use super::super::App;
use crate::app::platform;
use crate::app::ui_actions;
use crate::ui::{SelectedShapeProps, UiAction};
use astra_canvas::shapes::Shape;
use astra_canvas::tools::ToolKind;

use super::{gpu, pending, png_export, scene};

pub fn handle_redraw(app: &mut App) {
    let Some(state) = app.state.as_mut() else {
        return;
    };

    state.event_handler.update_laser_trail(1.0 / 60.0);
    if !state.event_handler.laser_trail.is_empty() {
        state.window.request_redraw();
    }
    pending::apply_pending_items(state);

    state.ui_state.current_tool = state.canvas.tool_manager.current_tool;
    state.ui_state.selection_count = state.canvas.selection.len();
    state.ui_state.zoom_level = state.canvas.camera.zoom;
    state.ui_state.grid_style = state.config.grid_style;

    if let Some(&shape_id) = state.canvas.selection.first() {
        if let Some(shape) = state.canvas.document.get_shape(shape_id) {
            let style = match shape {
                Shape::Group(g) => g
                    .children
                    .first()
                    .filter(|c| !matches!(c, Shape::Text(_)))
                    .or_else(|| g.children.first())
                    .map(|c| c.style())
                    .unwrap_or_else(|| shape.style()),
                _ => shape.style(),
            };
            state.ui_state.update_from_style(style);
        }
    }

    state.canvas.tool_manager.current_style = state.ui_state.to_shape_style();
    state.canvas.tool_manager.corner_radius = state.ui_state.corner_radius as f64;

    let selection_count = state.canvas.selection.len();
    let mut selected_props = if selection_count >= 1 {
        let shape_id = state.canvas.selection[0];
        if let Some(shape) = state.canvas.document.get_shape(shape_id) {
            SelectedShapeProps::from_shape_with_count(shape, selection_count)
        } else {
            SelectedShapeProps::default()
        }
    } else {
        SelectedShapeProps::default()
    };

    let current_tool = state.canvas.tool_manager.current_tool;
    let is_drawing_tool = matches!(
        current_tool,
        ToolKind::Rectangle
            | ToolKind::Diamond
            | ToolKind::Ellipse
            | ToolKind::Line
            | ToolKind::Arrow
            | ToolKind::Freehand
            | ToolKind::Highlighter
    );

    if is_drawing_tool && !selected_props.has_selection {
        selected_props = SelectedShapeProps::for_tool(current_tool, &state.ui_state);
    }

    platform::check_autosave(state);

    let text_selection_state: Option<(astra_canvas::shapes::ShapeId, std::ops::Range<usize>)> =
        if let (Some(text_id), Some(edit_state)) =
            (state.event_handler.editing_text, &state.text_edit_state)
        {
            edit_state.selection_range().map(|r| (text_id, r))
        } else {
            None
        };

    let egui_data = ui_actions::run_egui(state, &selected_props, &text_selection_state);

    let deferred = egui_data
        .deferred
        .or_else(|| app.state.as_mut().and_then(|s| s.deferred_action.take()));

    if let Some(action) = deferred {
        if let Some(render_cx) = app.render_cx.as_ref() {
            let Some(state) = app.state.as_mut() else {
                return;
            };
            let is_copy = matches!(action, UiAction::CopyPng);
            png_export::apply_png_export(state, render_cx, is_copy);
        }
    }

    let Some(state) = app.state.as_mut() else {
        return;
    };
    let scene = scene::build_render_scene(state);
    let Some(render_cx) = app.render_cx.as_ref() else {
        return;
    };
    let Some(state) = app.state.as_mut() else {
        return;
    };
    gpu::present_frame(
        state,
        render_cx,
        scene,
        egui_data.primitives,
        egui_data.textures_delta,
        egui_data.pixels_per_point,
        egui_data.action_taken,
    );
}
