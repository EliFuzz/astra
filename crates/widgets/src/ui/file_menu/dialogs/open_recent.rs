use super::super::super::{UiAction, UiState};
use crate::{default_btn, primary_btn};
use crate::theme;
use egui::{Align2, Context, CornerRadius, Frame, Margin, Pos2, Stroke, Vec2};

pub fn render_open_recent_dialog(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let mut action = None;

    egui::Area::new(egui::Id::new("open_recent_backdrop"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            let screen_rect = ctx.content_rect();
            let response = ui.allocate_rect(screen_rect, egui::Sense::click());
            ui.painter()
                .rect_filled(screen_rect, 0.0, theme::backdrop());
            if response.clicked() {
                ui_state.open_recent_dialog_open = false;
            }
        });

    egui::Area::new(egui::Id::new("open_recent_dialog"))
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
                                egui::RichText::new("Open Recent")
                                    .size(16.0)
                                    .strong()
                                    .color(theme::text()),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if default_btn(ui, "X") {
                                        ui_state.open_recent_dialog_open = false;
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
                                egui::RichText::new("No recent documents")
                                    .color(theme::text_muted()),
                            );
                        } else {
                            ui.add_space(4.0);

                            let selected_text = ui_state
                                .selected_recent_document
                                .as_deref()
                                .unwrap_or("Select a document")
                                .to_string();
                            ui.scope(|ui| {
                                ui.visuals_mut().widgets.inactive.bg_stroke =
                                    Stroke::new(0.5, theme::input_border());
                                ui.visuals_mut().widgets.hovered.bg_stroke =
                                    Stroke::new(0.5, theme::input_border_hover());
                                ui.visuals_mut().widgets.active.bg_stroke =
                                    Stroke::new(1.0, theme::accent());
                                ui.visuals_mut().widgets.inactive.weak_bg_fill = theme::input_bg();
                                ui.visuals_mut().widgets.hovered.weak_bg_fill = theme::input_bg();

                                egui::ComboBox::from_id_salt("recent_docs_dropdown")
                                    .selected_text(selected_text)
                                    .width(260.0)
                                    .show_ui(ui, |ui| {
                                        for doc_name in &ui_state.recent_documents.clone() {
                                            ui.selectable_value(
                                                &mut ui_state.selected_recent_document,
                                                Some(doc_name.clone()),
                                                doc_name,
                                            );
                                        }
                                    });
                            });

                            ui.add_space(12.0);

                            ui.horizontal(|ui| {
                                if primary_btn(ui, "Open") {
                                    if let Some(doc_name) = &ui_state.selected_recent_document {
                                        action = Some(UiAction::LoadLocal(doc_name.clone()));
                                        ui_state.open_recent_dialog_open = false;
                                    }
                                }
                            });
                        }
                    });
                });
        });

    action
}
