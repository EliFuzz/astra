#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use native as imp;

#[cfg(target_arch = "wasm32")]
use wasm as imp;

pub(super) fn read_clipboard_text() -> Option<String> {
    imp::read_clipboard_text()
}

pub(super) fn render_platform_menu_items(
    ui: &mut egui::Ui,
    ui_state: &mut super::UiState,
) -> Option<super::UiAction> {
    imp::render_platform_menu_items(ui, ui_state)
}
