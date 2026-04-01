use egui::{CornerRadius, Pos2, Rect, Vec2};

use crate::theme;

pub(super) fn hamburger_button(ui: &mut egui::Ui, is_open: bool) -> bool {
    let size = Vec2::new(28.0, 28.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let bg_color = if is_open {
            theme::selected_bg()
        } else if response.hovered() {
            theme::hover_bg()
        } else {
            egui::Color32::TRANSPARENT
        };

        let line_color = if is_open {
            theme::accent()
        } else if response.hovered() {
            theme::icon_hover_color()
        } else {
            theme::icon_color()
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(6), bg_color);

        let line_width = 12.0;
        let line_height = 1.5;
        let spacing = 3.5;
        let start_x = rect.center().x - line_width / 2.0;
        let center_y = rect.center().y;

        for i in -1..=1 {
            let y = center_y + (i as f32) * spacing;
            let line_rect = Rect::from_min_size(
                Pos2::new(start_x, y - line_height / 2.0),
                Vec2::new(line_width, line_height),
            );
            ui.painter()
                .rect_filled(line_rect, CornerRadius::same(1), line_color);
        }
    }

    response.on_hover_text("Menu").clicked()
}
