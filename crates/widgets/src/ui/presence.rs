use super::{UiAction, UiState};
use crate::{default_btn, primary_btn};
use crate::theme;
use egui::{Align2, Context, Frame, Pos2, Vec2};

pub fn render_math_editor(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let (shape_id, latex_input) = ui_state.math_editor.as_mut()?;
    let shape_id = *shape_id;
    let mut action: Option<UiAction> = None;
    let mut close = false;

    egui::Area::new(egui::Id::new("math_editor_backdrop"))
        .fixed_pos(Pos2::ZERO)
        .order(egui::Order::Background)
        .show(ctx, |ui| {
            let screen_rect = ctx.content_rect();
            let response = ui.allocate_rect(screen_rect, egui::Sense::click());
            ui.painter()
                .rect_filled(screen_rect, 0.0, theme::backdrop());
            if response.clicked() {
                close = true;
            }
        });

    egui::Area::new(egui::Id::new("math_editor_dialog"))
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            Frame::new()
                .fill(theme::dialog_bg())
                .corner_radius(egui::CornerRadius::same(14))
                .stroke(egui::Stroke::NONE)
                .shadow(egui::epaint::Shadow {
                    spread: 0,
                    blur: 36,
                    offset: [0, 12],
                    color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 50),
                })
                .inner_margin(egui::Margin::same(20))
                .show(ui, |ui| {
                    ui.set_width(400.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Edit Equation")
                                    .size(15.0)
                                    .strong()
                                    .color(theme::text()),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if default_btn(ui, "X") {
                                        close = true;
                                    }
                                },
                            );
                        });

                        ui.add_space(12.0);

                        ui.label(
                            egui::RichText::new("LaTeX:")
                                .size(12.0)
                                .color(theme::text_muted()),
                        );
                        ui.add_space(4.0);

                        let text_edit = egui::TextEdit::multiline(latex_input)
                            .desired_width(f32::INFINITY)
                            .desired_rows(3)
                            .font(egui::TextStyle::Monospace);
                        let response = ui.add(text_edit);
                        response.request_focus();

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if ui.small_button("Copy").clicked() {
                                action = Some(UiAction::CopyText(latex_input.clone()));
                            }
                            if ui.small_button("Paste").clicked() {
                                match super::platform::read_clipboard_text() {
                                    Some(text) => *latex_input = text,
                                    None => action = Some(UiAction::RequestMathPaste),
                                }
                            }
                        });

                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(
                                "Examples: x^2",
                            )
                            .size(11.0)
                            .color(theme::text_muted()),
                        );

                        ui.add_space(16.0);

                        ui.horizontal(|ui| {
                            if primary_btn(ui, "Apply") {
                                action =
                                    Some(UiAction::UpdateMathLatex(shape_id, latex_input.clone()));
                                close = true;
                            }
                            ui.add_space(8.0);
                            if default_btn(ui, "Cancel") {
                                close = true;
                            }
                        });
                    });
                });
        });

    if close {
        ui_state.math_editor = None;
    }

    action
}
