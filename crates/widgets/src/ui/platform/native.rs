pub(super) fn read_clipboard_text() -> Option<String> {
    arboard::Clipboard::new()
        .ok()
        .and_then(|mut cb| cb.get_text().ok())
}

pub(super) fn render_platform_menu_items(
    _ui: &mut egui::Ui,
    _ui_state: &mut super::super::UiState,
) -> Option<super::super::UiAction> {
    None
}
