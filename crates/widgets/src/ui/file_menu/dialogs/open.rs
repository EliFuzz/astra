use super::super::super::{UiAction, UiState};
use crate::default_btn;
use crate::theme;
use egui::{Align2, Context, CornerRadius, Frame, Margin, Pos2, Stroke, Vec2};

pub fn render_open_dialog(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let mut action = None;

    egui::Area::new(egui::Id::new("open_dialog_backdrop"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            let screen_rect = ctx.content_rect();
            let response = ui.allocate_rect(screen_rect, egui::Sense::click());
            ui.painter()
                .rect_filled(screen_rect, 0.0, theme::backdrop());
            if response.clicked() {
                ui_state.open_dialog_open = false;
            }
        });

    egui::Area::new(egui::Id::new("open_dialog"))
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
                                egui::RichText::new("Open Document")
                                    .size(15.0)
                                    .strong()
                                    .color(theme::text()),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if default_btn(ui, "X") {
                                        ui_state.open_dialog_open = false;
                                    }
                                },
                            );
                        });

                        ui.add_space(12.0);

                        ui.label(
                            egui::RichText::new("Select document:")
                                .size(12.0)
                                .color(theme::text_muted()),
                        );

                        if ui_state.recent_documents.is_empty() {
                            ui.label(
                                egui::RichText::new("No saved documents")
                                    .color(theme::text_muted()),
                            );
                        } else {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    for doc_name in &ui_state.recent_documents.clone() {
                                        if ui.button(doc_name).clicked() {
                                            action = Some(UiAction::LoadLocal(doc_name.clone()));
                                            ui_state.open_dialog_open = false;
                                        }
                                    }
                                });
                        }
                    });
                });
        });

    action
}
