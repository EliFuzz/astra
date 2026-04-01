use super::{
    SelectedShapeProps, UiAction, UiState, bottom_toolbar, file_menu, presence, properties,
    toolbar,
};
use crate::theme;

pub fn render_ui(
    ctx: &egui::Context,
    ui_state: &mut UiState,
    selected_props: &SelectedShapeProps,
) -> Option<UiAction> {
    theme::apply_to_egui(ctx);
    egui_extras::install_image_loaders(ctx);

    let toolbar_action = toolbar::render_toolbar(ctx, ui_state);
    let properties_action = properties::render_properties_panel(ctx, ui_state, selected_props);
    let file_menu_action = file_menu::render_file_menu(ctx, ui_state);
    let bottom_toolbar_action = bottom_toolbar::render_bottom_toolbar(ctx, ui_state);
    let math_editor_action = presence::render_math_editor(ctx, ui_state);

    toolbar_action
        .or(properties_action)
        .or(file_menu_action)
        .or(bottom_toolbar_action)
        .or(math_editor_action)
}
