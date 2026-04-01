use super::super::{UiAction, UiState};
use crate::{IconButton, theme};
use crate::icon;

pub(super) fn render_zoom_section(ui: &mut egui::Ui, state: &UiState) -> Option<UiAction> {
    let mut action = None;
    let text_color = theme::icon_color();

    let minus_response = ui.add(
        egui::Label::new(egui::RichText::new("\u{2212}").size(16.0).color(text_color))
            .sense(egui::Sense::click()),
    );
    if minus_response.clicked() {
        action = Some(UiAction::ZoomOut);
    }
    minus_response.clone().on_hover_text("Zoom out");
    minus_response.on_hover_cursor(egui::CursorIcon::PointingHand);

    ui.add_space(12.0);

    let zoom_pct = (state.zoom_level / astra_canvas::camera::BASE_ZOOM * 100.0).round() as i32;
    let zoom_response = ui.add(
        egui::Label::new(
            egui::RichText::new(format!("{}%", zoom_pct))
                .size(13.0)
                .color(text_color),
        )
        .sense(egui::Sense::click()),
    );
    if zoom_response.clicked() {
        action = Some(UiAction::ZoomReset);
    }
    zoom_response.clone().on_hover_text("Reset to 100%");
    zoom_response.on_hover_cursor(egui::CursorIcon::PointingHand);

    ui.add_space(12.0);

    let plus_response = ui.add(
        egui::Label::new(egui::RichText::new("+").size(16.0).color(text_color))
            .sense(egui::Sense::click()),
    );
    if plus_response.clicked() {
        action = Some(UiAction::ZoomIn);
    }
    plus_response.clone().on_hover_text("Zoom in");
    plus_response.on_hover_cursor(egui::CursorIcon::PointingHand);

    ui.add_space(8.0);

    let fit_tooltip = if state.selection_count > 0 {
        "Zoom to fit selection"
    } else {
        "Zoom to fit all elements"
    };
    if IconButton::new(
        icon!("zoom-fit.png"),
        fit_tooltip,
    )
    .small()
    .show(ui)
    {
        action = Some(UiAction::ZoomToFit);
    }

    action
}
