use super::super::AppState;
use super::super::platform;
use crate::ui::UiAction;
use astra_canvas::tools::ToolKind;

pub(super) fn apply(state: &mut AppState, action: &UiAction) {
    let should_persist = matches!(
        action,
        UiAction::ToggleGrid
            | UiAction::ToggleGridSnap
            | UiAction::ToggleSmartSnap
            | UiAction::ToggleAngleSnap
            | UiAction::SetBgColor(_)
            | UiAction::ToggleTheme
    );

    match action {
        UiAction::SetTool(tool) => {
            if *tool == ToolKind::InsertImage {
                platform::insert_image();
            } else {
                state.canvas.clear_selection();
                state.canvas.set_tool(*tool);
            }
        }
        UiAction::ToggleGrid => {
            state.config.grid_style = state.config.grid_style.next();
            state.ui_state.grid_style = state.config.grid_style;
        }
        UiAction::ZoomIn => {
            let center = kurbo::Point::new(
                state.canvas.viewport_size.width / 2.0,
                state.canvas.viewport_size.height / 2.0,
            );
            state.canvas.camera.zoom_at(center, 1.25);
            state.ui_state.zoom_level = state.canvas.camera.zoom;
        }
        UiAction::ZoomOut => {
            let center = kurbo::Point::new(
                state.canvas.viewport_size.width / 2.0,
                state.canvas.viewport_size.height / 2.0,
            );
            state.canvas.camera.zoom_at(center, 0.8);
            state.ui_state.zoom_level = state.canvas.camera.zoom;
        }
        UiAction::ZoomReset => {
            state.canvas.camera.zoom = astra_canvas::camera::BASE_ZOOM;
            state.ui_state.zoom_level = astra_canvas::camera::BASE_ZOOM;
        }
        UiAction::ToggleGridSnap => {
            state.ui_state.grid_snap_enabled = !state.ui_state.grid_snap_enabled;
        }
        UiAction::ToggleSmartSnap => {
            state.ui_state.smart_snap_enabled = !state.ui_state.smart_snap_enabled;
        }
        UiAction::ToggleAngleSnap => {
            state.ui_state.angle_snap_enabled = !state.ui_state.angle_snap_enabled;
        }
        UiAction::ZoomToFit => {
            let bounds = if state.canvas.selection.is_empty() {
                state.canvas.document.bounds()
            } else {
                state.canvas.selection_bounds()
            };
            if let Some(bounds) = bounds {
                state
                    .canvas
                    .camera
                    .fit_to_bounds(bounds, state.canvas.viewport_size, 50.0);
            } else {
                state.canvas.camera.reset();
            }
            state.ui_state.zoom_level = state.canvas.camera.zoom;
        }
        UiAction::SetBgColor(color) => {
            state.ui_state.bg_color = *color;
            state.config.background_color =
                peniko::Color::from_rgba8(color.r(), color.g(), color.b(), color.a());
        }
        UiAction::SetExportScale(scale) => {
            state.ui_state.export_scale = *scale;
        }
        UiAction::ToggleTheme => {
            astra_widgets::theme::toggle();
            let bg = astra_widgets::theme::canvas_bg();
            state.ui_state.bg_color = bg;
            state.config.background_color =
                peniko::Color::from_rgba8(bg.r(), bg.g(), bg.b(), bg.a());
        }
        UiAction::CopyText(text) => {
            platform::copy_text(text);
        }
        UiAction::RequestMathPaste => {
            platform::request_clipboard_for_math();
        }
        _ => {}
    }

    if should_persist {
        crate::app::preferences::save(state);
    }
}
