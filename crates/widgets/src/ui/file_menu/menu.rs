use super::hamburger::hamburger_button;
use super::super::{UiAction, UiState};
use super::dialogs::{render_open_dialog, render_open_recent_dialog, render_save_dialog};
use crate::{
    menu_item as widgets_menu_item,
    menu_separator as widgets_menu_separator,
};
use crate::theme;
use egui::{Align2, Color32, Context, Pos2, Rect, Vec2};

pub fn render_file_menu(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let mut action = None;

    egui::Area::new(egui::Id::new("hamburger_button"))
        .anchor(Align2::LEFT_TOP, Vec2::new(12.0, 12.0))
        .show(ctx, |ui| {
            crate::panel_frame().show(ui, |ui| {
                if hamburger_button(ui, ui_state.menu_open) {
                    ui_state.menu_open = !ui_state.menu_open;
                }
            });
        });

    if ui_state.menu_open {
        egui::Area::new(egui::Id::new("file_menu_dropdown"))
            .anchor(Align2::LEFT_TOP, Vec2::new(12.0, 56.0))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                crate::panel_frame().show(ui, |ui| {
                    ui.set_width(195.0);
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(0.0, 2.0);

                        if menu_item(ui, "Save", "Ctrl+S") {
                            action = Some(UiAction::SaveLocal);
                            ui_state.menu_open = false;
                        }

                        widgets_menu_separator(ui);

                        if menu_item(ui, "Clear", "") {
                            action = Some(UiAction::ClearDocument);
                            ui_state.menu_open = false;
                        }

                        widgets_menu_separator(ui);

                        if let Some(platform_action) = super::super::platform::render_platform_menu_items(ui, ui_state) {
                            action = Some(platform_action);
                        }

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(0.0, 0.0);
                            ui.add_space(12.0);
                            let text_response = ui.add(
                                egui::Label::new(
                                    egui::RichText::new("Export PNG")
                                        .size(12.0)
                                        .color(theme::text()),
                                )
                                .sense(egui::Sense::click()),
                            );

                            ui.add_space(6.0);
                            for scale in [1u8, 2, 3] {
                                let label = format!("{}x", scale);
                                let selected = ui_state.export_scale == scale;
                                let btn = egui::Button::new(
                                    egui::RichText::new(&label).size(11.0).color(if selected {
                                        theme::accent()
                                    } else {
                                        theme::icon_color()
                                    }),
                                )
                                .fill(if selected {
                                    theme::selected_bg()
                                } else {
                                    Color32::TRANSPARENT
                                })
                                .stroke(egui::Stroke::NONE)
                                .corner_radius(egui::CornerRadius::same(4))
                                .min_size(Vec2::new(24.0, 20.0));
                                if ui.add(btn).clicked() {
                                    action = Some(UiAction::SetExportScale(scale));
                                }
                            }

                            ui.add_space(6.0);
                            ui.label(
                                egui::RichText::new("Ctrl+E")
                                    .size(10.0)
                                    .color(theme::text_muted()),
                            );

                            if text_response.clicked() {
                                action = Some(UiAction::ExportPng);
                                ui_state.menu_open = false;
                            }
                        });
                    });
                });
            });

        if ctx.input(|i| i.pointer.any_click()) {
            let buttons_rect = Rect::from_min_size(Pos2::new(12.0, 12.0), Vec2::new(44.0, 48.0));
            let menu_rect = Rect::from_min_size(Pos2::new(12.0, 56.0), Vec2::new(180.0, 250.0));
            if let Some(pos) = ctx.input(|i| i.pointer.interact_pos()) {
                if !buttons_rect.contains(pos) && !menu_rect.contains(pos) {
                    ui_state.menu_open = false;
                }
            }
        }
    }

    if ui_state.save_dialog_open {
        if let Some(save_action) = render_save_dialog(ctx, ui_state) {
            action = Some(save_action);
        }
    }

    if ui_state.open_dialog_open {
        if let Some(open_action) = render_open_dialog(ctx, ui_state) {
            action = Some(open_action);
        }
    }

    if ui_state.open_recent_dialog_open {
        if let Some(open_action) = render_open_recent_dialog(ctx, ui_state) {
            action = Some(open_action);
        }
    }

    action
}

fn menu_item(ui: &mut egui::Ui, label: &str, shortcut: &str) -> bool {
    widgets_menu_item(ui, label, shortcut)
}

