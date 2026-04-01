use super::super::AppState;
use crate::ui::UiAction;

pub(super) fn apply(state: &mut AppState, action: &UiAction) {
    match action {
        UiAction::BringToFront => state.canvas.bring_to_front_selected(),
        UiAction::SendToBack => state.canvas.send_to_back_selected(),
        UiAction::BringForward => state.canvas.bring_forward_selected(),
        UiAction::SendBackward => state.canvas.send_backward_selected(),
        _ => {}
    }
}
