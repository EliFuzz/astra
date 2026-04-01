use super::super::AppState;
use crate::ui::UiAction;
use astra_canvas::AlignMode;

pub(super) fn apply(state: &mut AppState, action: &UiAction) {
    let mode = match action {
        UiAction::AlignLeft => AlignMode::Left,
        UiAction::AlignRight => AlignMode::Right,
        UiAction::AlignTop => AlignMode::Top,
        UiAction::AlignBottom => AlignMode::Bottom,
        UiAction::AlignCenterH => AlignMode::CenterHorizontal,
        UiAction::AlignCenterV => AlignMode::CenterVertical,
        _ => return,
    };
    state.canvas.align_selected(mode);
}
