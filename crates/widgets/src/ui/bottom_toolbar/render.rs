use super::super::{UiAction, UiState};
use super::{options, zoom};
use crate::theme;
use egui::{Context, Pos2, Rect, Vec2};

pub fn render_bottom_toolbar(ctx: &Context, ui_state: &mut UiState) -> Option<UiAction> {
    let mut action = None;
    let mut bg_color_rect = Rect::NOTHING;

    let screen_rect = ctx.content_rect();
    let toolbar_height = 36.0;
    let margin = 12.0;
    let bottom_y = screen_rect.max.y - margin - toolbar_height;

    egui::Area::new(egui::Id::new("bottom_toolbar"))
        .fixed_pos(Pos2::new(margin, bottom_y.max(margin)))
        .interactable(true)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            crate::toolbar_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);

                    if let Some(a) = options::render_history_section(ui) {
                        action = Some(a);
                    }

                    separator(ui);

                    let (grid_action, rect) = options::render_grid_and_bg_section(ui, ui_state);
                    bg_color_rect = rect;
                    if let Some(a) = grid_action {
                        action = Some(a);
                    }

                    separator(ui);

                    if let Some(a) = zoom::render_zoom_section(ui, ui_state) {
                        action = Some(a);
                    }

                    separator(ui);

                    if let Some(a) = options::render_snap_section(ui, ui_state) {
                        action = Some(a);
                    }
                });
            });
        });

    if let Some(a) = options::render_bg_color_popover(ctx, ui_state, bg_color_rect) {
        action = Some(a);
    }

    action
}
fn separator(ui: &mut egui::Ui) {
    ui.add_space(8.0);
    ui.label(
        egui::RichText::new("|")
            .size(14.0)
            .color(theme::separator_color()),
    );
    ui.add_space(8.0);
}

