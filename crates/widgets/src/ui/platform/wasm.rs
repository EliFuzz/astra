use super::super::{UiAction, UiState};

pub(super) fn read_clipboard_text() -> Option<String> {
    None
}

pub(super) fn render_platform_menu_items(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
) -> Option<UiAction> {
    let mut action = None;
    if crate::menu_item(ui, "Export", "") {
        action = Some(UiAction::DownloadDocument);
        ui_state.menu_open = false;
    }
    if crate::menu_item(ui, "Import", "") {
        action = Some(UiAction::UploadDocument);
        ui_state.menu_open = false;
    }
    crate::menu_separator(ui);
    action
}
