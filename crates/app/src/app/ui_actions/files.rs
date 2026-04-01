use super::super::AppState;
use super::super::platform;
use crate::ui::UiAction;

pub(super) fn apply(state: &mut AppState, action: &UiAction) {
    match action {
        UiAction::SaveLocal => {
            platform::save_document(&state.canvas.document, &state.canvas.document.name);
        }
        UiAction::SaveLocalAs => {
            state
                .ui_state
                .open_save_dialog(state.canvas.document.name.clone());
        }
        UiAction::ShowOpenDialog => {
            state.ui_state.open_document_dialog(false);
            platform::list_documents();
        }
        UiAction::ShowOpenRecentDialog => {
            state.ui_state.open_document_dialog(true);
            platform::list_documents();
        }
        UiAction::SaveLocalWithName(name) => {
            state.canvas.document.name = name.clone();
            platform::save_document_with_name(&mut state.canvas.document, name);
            state.ui_state.remember_recent_document(name);
        }
        UiAction::LoadLocal(name) => {
            platform::load_document_by_name(name);
        }
        UiAction::SaveDocument => {
            platform::save_document(&state.canvas.document, &state.canvas.document.name);
        }
        UiAction::LoadDocument => {
            platform::load_document();
        }
        UiAction::DownloadDocument => {
            platform::download_document(&state.canvas.document, &state.canvas.document.name);
        }
        UiAction::UploadDocument => {
            platform::upload_document();
        }
        _ => {}
    }
}
