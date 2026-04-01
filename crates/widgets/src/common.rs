use egui::{Color32, CornerRadius, Stroke, TextEdit, Ui, Vec2};

use crate::theme;

pub fn primary_btn(ui: &mut Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(egui::RichText::new(text).size(12.0).color(Color32::WHITE))
            .fill(theme::primary_button_bg())
            .stroke(Stroke::NONE)
            .min_size(Vec2::new(72.0, 28.0))
            .corner_radius(CornerRadius::same(6)),
    )
    .clicked()
}

pub fn secondary_btn(ui: &mut Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .size(12.0)
                .color(theme::secondary_button_text()),
        )
        .fill(theme::secondary_button_bg())
        .stroke(Stroke::NONE)
        .min_size(Vec2::new(72.0, 28.0))
        .corner_radius(CornerRadius::same(6)),
    )
    .clicked()
}

pub fn default_btn(ui: &mut Ui, text: &str) -> bool {
    ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .size(14.0)
                .color(theme::text_muted()),
        )
        .frame(false),
    )
    .clicked()
}

pub fn input_text(ui: &mut Ui, text: &mut String, width: f32, hint: &str) -> egui::Response {
    ui.scope(|ui| {
        ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::new(0.5, theme::input_border());
        ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::new(0.5, theme::input_border_hover());
        ui.visuals_mut().widgets.active.bg_stroke = Stroke::new(1.0, theme::accent());

        ui.add(
            TextEdit::singleline(text)
                .desired_width(width)
                .text_color(theme::input_text_color())
                .background_color(theme::input_bg())
                .hint_text(hint),
        )
    })
    .inner
}
