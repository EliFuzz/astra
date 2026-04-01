use super::super::AppState;
use super::super::platform;
use crate::ui::UiAction;

pub(super) fn apply(state: &mut AppState, action: &UiAction) {
    match action {
        UiAction::ClearDocument => {
            if !state.canvas.document.is_empty() {
                state.canvas.document.push_undo();
                state.canvas.document.clear();
                state.canvas.clear_selection();
            }
        }
        UiAction::Undo => {
            if state.canvas.document.undo() {
                state.canvas.clear_selection();
            }
        }
        UiAction::Redo => {
            if state.canvas.document.redo() {
                state.canvas.clear_selection();
            }
        }
        UiAction::Duplicate => {
            state.canvas.duplicate_selected();
        }
        UiAction::CopyShapes => {
            state.ui_state.clipboard_shapes = state.canvas.copy_selected_as_json();
        }
        UiAction::CutShapes => {
            state.ui_state.clipboard_shapes = state.canvas.cut_selected_as_json();
        }
        UiAction::PasteShapes => {
            let from_internal_clipboard = state
                .ui_state
                .clipboard_shapes
                .as_deref()
                .and_then(|json| {
                    state
                        .canvas
                        .paste_shapes_from_json(json, kurbo::Vec2::new(20.0, 20.0))
                })
                .is_some();
            if !from_internal_clipboard {
                platform::paste_from_external_clipboard(state);
            }
        }
        UiAction::UpdateMathLatex(shape_id, latex) => {
            state.canvas.update_math_latex(*shape_id, latex);
        }
        UiAction::SelectAll => {
            state.canvas.select_all();
        }
        UiAction::DeleteSelected => {
            if !state.canvas.selection.is_empty() {
                state.canvas.document.push_undo();
                state.canvas.delete_selected();
            }
        }
        UiAction::GroupSelected => {
            state.canvas.group_selected();
        }
        UiAction::UngroupSelected => {
            state.canvas.ungroup_selected();
        }
        UiAction::NudgeSelection(dx, dy) => {
            state.canvas.nudge_selected(kurbo::Vec2::new(*dx, *dy));
        }
        _ => {}
    }
}
