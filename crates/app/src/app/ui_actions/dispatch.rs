use super::super::AppState;
use crate::ui::{SelectedShapeProps, UiAction, render_ui};

pub struct EguiRenderData {
    pub primitives: Vec<egui::ClippedPrimitive>,
    pub textures_delta: egui::TexturesDelta,
    pub pixels_per_point: f32,
    pub action_taken: bool,
    pub deferred: Option<UiAction>,
}

pub fn run_egui(
    state: &mut AppState,
    selected_props: &SelectedShapeProps,
    text_selection_state: &Option<(astra_canvas::shapes::ShapeId, std::ops::Range<usize>)>,
) -> EguiRenderData {
    let egui_input = state.egui_state.take_egui_input(&state.window);

    let mut gathered_action: Option<UiAction> = None;
    #[expect(deprecated)]
    let egui_output = state.egui_ctx.run(egui_input, |ctx| {
        gathered_action = render_ui(ctx, &mut state.ui_state, selected_props);
    });

    state
        .egui_state
        .handle_platform_output(&state.window, egui_output.platform_output);
    let primitives = state
        .egui_ctx
        .tessellate(egui_output.shapes, egui_output.pixels_per_point);

    let mut action_taken = false;
    let mut deferred: Option<UiAction> = None;

    if let Some(action) = gathered_action {
        action_taken = true;
        match action.clone() {
            UiAction::ExportPng | UiAction::CopyPng => {
                deferred = Some(action);
            }
            _ => apply_action(state, &action, text_selection_state),
        }
    }

    EguiRenderData {
        primitives,
        textures_delta: egui_output.textures_delta,
        pixels_per_point: egui_output.pixels_per_point,
        action_taken,
        deferred,
    }
}

fn apply_action(
    state: &mut AppState,
    action: &UiAction,
    text_selection_state: &Option<(astra_canvas::shapes::ShapeId, std::ops::Range<usize>)>,
) {
    match action {
        UiAction::SetTool(_)
        | UiAction::ToggleGrid
        | UiAction::ZoomIn
        | UiAction::ZoomOut
        | UiAction::ZoomReset
        | UiAction::ToggleGridSnap
        | UiAction::ToggleSmartSnap
        | UiAction::ToggleAngleSnap
        | UiAction::ZoomToFit
        | UiAction::SetBgColor(_)
        | UiAction::SetExportScale(_)
        | UiAction::ToggleTheme => super::view::apply(state, action),

        UiAction::SetStrokeColor(_)
        | UiAction::SetFillColor(_)
        | UiAction::SetStrokeWidth(_)
        | UiAction::SetCornerRadius(_)
        | UiAction::SetSloppiness(_)
        | UiAction::SetFillPattern(_)
        | UiAction::SetPathStyle(_)
        | UiAction::SetStrokeStyle(_)
        | UiAction::SetOpacity(_)
        | UiAction::SetFontSize(_) => super::style::apply(state, action, text_selection_state),

        UiAction::SaveLocal
        | UiAction::SaveLocalAs
        | UiAction::ShowOpenDialog
        | UiAction::ShowOpenRecentDialog
        | UiAction::SaveLocalWithName(_)
        | UiAction::LoadLocal(_)
        | UiAction::SaveDocument
        | UiAction::LoadDocument
        | UiAction::DownloadDocument
        | UiAction::UploadDocument => super::files::apply(state, action),

        UiAction::ClearDocument
        | UiAction::Undo
        | UiAction::Redo
        | UiAction::Duplicate
        | UiAction::CopyShapes
        | UiAction::CutShapes
        | UiAction::PasteShapes
        | UiAction::UpdateMathLatex(_, _)
        | UiAction::SelectAll
        | UiAction::DeleteSelected
        | UiAction::GroupSelected
        | UiAction::UngroupSelected
        | UiAction::NudgeSelection(_, _) => super::document::apply(state, action),

        UiAction::BringToFront
        | UiAction::SendToBack
        | UiAction::BringForward
        | UiAction::SendBackward => super::ordering::apply(state, action),

        UiAction::AlignLeft
        | UiAction::AlignRight
        | UiAction::AlignTop
        | UiAction::AlignBottom
        | UiAction::AlignCenterH
        | UiAction::AlignCenterV => super::align::apply(state, action),

        UiAction::ExportPng | UiAction::CopyPng => {}

        UiAction::CopyText(_) | UiAction::RequestMathPaste => super::view::apply(state, action),
    }
}

pub(in crate::app) fn dispatch_action_simple(state: &mut AppState, action: &UiAction) {
    apply_action(state, action, &None);
}
