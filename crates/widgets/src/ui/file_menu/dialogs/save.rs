use super::super::super::{UiAction, UiState};
use crate::{default_btn, input_text, primary_btn, secondary_btn};
use crate::theme;
use egui::{Align2, Context, CornerRadius, Frame, Margin, Pos2, Stroke, Vec2};

pub fn render_save_dialog(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let mut action = None;

    egui::Area::new(egui::Id::new("save_dialog_backdrop"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            let screen_rect = ctx.content_rect();
            let response = ui.allocate_rect(screen_rect, egui::Sense::click());
            ui.painter()
                .rect_filled(screen_rect, 0.0, theme::backdrop());
            if response.clicked() {
                ui_state.save_dialog_open = false;
            }
        });

    egui::Area::new(egui::Id::new("save_dialog"))
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            Frame::new()
                .fill(theme::dialog_bg())
                .corner_radius(CornerRadius::same(14))
                .stroke(Stroke::NONE)
                .shadow(egui::epaint::Shadow {
                    spread: 0,
                    blur: 36,
                    offset: [0, 12],
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 50),
                })
                .inner_margin(Margin::same(20))
                .show(ui, |ui| {
                    ui.set_width(300.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Save Document")
                                    .size(15.0)
                                    .strong()
                                    .color(theme::text()),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if default_btn(ui, "X") {
                                        ui_state.save_dialog_open = false;
                                    }
                                },
                            );
                        });

                        ui.add_space(12.0);

                        ui.label(
                            egui::RichText::new("Document name:")
                                .size(12.0)
                                .color(theme::text_muted()),
                        );
                        let response = input_text(ui, &mut ui_state.save_name_input, 300.0, "");

                        if response.lost_focus()
                            && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            && !ui_state.save_name_input.trim().is_empty()
                        {
                            action = Some(UiAction::SaveLocalWithName(
                                ui_state.save_name_input.trim().to_string(),
                            ));
                            ui_state.save_dialog_open = false;
                        }

                        ui.add_space(12.0);

                        ui.horizontal(|ui| {
                            if secondary_btn(ui, "Cancel") {
                                ui_state.save_dialog_open = false;
                            }
                            if primary_btn(ui, "Save")
                                && !ui_state.save_name_input.trim().is_empty()
                            {
                                action = Some(UiAction::SaveLocalWithName(
                                    ui_state.save_name_input.trim().to_string(),
                                ));
                                ui_state.save_dialog_open = false;
                            }
                        });
                    });
                });
        });

    action
}
