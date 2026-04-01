use egui::{
    Color32, CornerRadius, CursorIcon, Frame, Margin, Pos2, Sense, Stroke, Ui, vec2,
};

use crate::{sizing, theme};

pub fn menu_item(ui: &mut Ui, label: &str, shortcut: &str) -> bool {
    menu_item_inner(ui, label, shortcut, true)
}

fn menu_item_inner(ui: &mut Ui, label: &str, shortcut: &str, enabled: bool) -> bool {
    let size = vec2(ui.available_width(), 26.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if ui.is_rect_visible(rect) {
        let bg_color = if !enabled {
            Color32::TRANSPARENT
        } else if response.hovered() {
            theme::hover_bg()
        } else {
            Color32::TRANSPARENT
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(sizing::CORNER_RADIUS), bg_color);

        let text_color = if enabled {
            theme::text()
        } else {
            theme::text_muted()
        };

        ui.painter().text(
            Pos2::new(rect.left() + 10.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(12.5),
            text_color,
        );

        if !shortcut.is_empty() {
            let shortcut_color = if enabled {
                theme::text_muted()
            } else {
                theme::separator_color()
            };
            ui.painter().text(
                Pos2::new(rect.right() - 10.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                shortcut,
                egui::FontId::proportional(11.0),
                shortcut_color,
            );
        }
    }

    let clicked = response.clicked();
    if enabled {
        response.on_hover_cursor(CursorIcon::PointingHand);
    }
    enabled && clicked
}

pub fn menu_separator(ui: &mut Ui) {
    ui.add_space(3.0);
    let rect = ui.available_rect_before_wrap();
    let y = rect.top();
    ui.painter().line_segment(
        [
            Pos2::new(rect.left() + 6.0, y),
            Pos2::new(rect.right() - 6.0, y),
        ],
        Stroke::new(0.5, theme::separator_color()),
    );
    ui.add_space(3.0);
}

pub fn panel_frame() -> Frame {
    Frame::new()
        .fill(theme::panel_bg())
        .corner_radius(CornerRadius::same(sizing::PANEL_RADIUS))
        .stroke(Stroke::NONE)
        .shadow(egui::epaint::Shadow {
            spread: 0,
            blur: 24,
            offset: [0, 6],
            color: Color32::from_rgba_premultiplied(0, 0, 0, 40),
        })
        .inner_margin(Margin::same(6))
}

pub fn toolbar_frame() -> Frame {
    Frame::new()
        .fill(theme::panel_bg())
        .corner_radius(CornerRadius::same(sizing::PANEL_RADIUS))
        .stroke(Stroke::NONE)
        .shadow(egui::epaint::Shadow {
            spread: 0,
            blur: 16,
            offset: [0, 3],
            color: Color32::from_rgba_premultiplied(0, 0, 0, 30),
        })
        .inner_margin(Margin::symmetric(10, 5))
}
