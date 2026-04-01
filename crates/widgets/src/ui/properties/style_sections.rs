use super::super::color_helpers::color_swatch_current;
use super::super::{ColorPopover, SelectedShapeProps, UiAction, UiState};
use crate::{
    COLORS, ColorSwatch, IconButton, NoColorSwatch,
    section_label as widgets_section_label, vertical_separator as widgets_vertical_separator,
};
use crate::icon;
use egui::{Color32, Rect, Vec2};

const QUICK_STROKE_COLORS: &[usize] = &[10, 0, 6, 2, 13, 17];
const QUICK_FILL_COLORS: &[usize] = &[10, 0, 6, 2, 13];

pub(super) fn render_stroke_section(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    _props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
    stroke_current_rect: &mut Rect,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Stroke");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);

            for &idx in QUICK_STROKE_COLORS {
                let color = COLORS[idx].shades[6];
                let is_selected = ui_state.stroke_color == color;
                if color_swatch_selectable(ui, color, COLORS[idx].name, is_selected) {
                    *action = Some(UiAction::SetStrokeColor(color));
                    ui_state.color_popover = ColorPopover::None;
                }
            }

            ui.add_space(4.0);
            widgets_vertical_separator(ui);
            ui.add_space(4.0);

            let (clicked, rect) = color_swatch_current(ui, ui_state.stroke_color, "Pick color");
            *stroke_current_rect = rect;
            if clicked {
                ui_state.color_popover =
                    if ui_state.color_popover == ColorPopover::StrokeFull {
                        ColorPopover::None
                    } else {
                        ColorPopover::StrokeFull
                    };
            }
        });
    });
}

pub(super) fn render_fill_section(
    ui: &mut egui::Ui,
    ui_state: &mut UiState,
    action: &mut Option<UiAction>,
    fill_current_rect: &mut Rect,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Background");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);

            let is_none_selected = ui_state.fill_color.is_none();
            if fill_swatch(ui, None, "None", is_none_selected) {
                *action = Some(UiAction::SetFillColor(None));
                ui_state.color_popover = ColorPopover::None;
            }

            for &idx in QUICK_FILL_COLORS {
                let color = COLORS[idx].shades[1];
                let is_selected = ui_state.fill_color == Some(color);
                if color_swatch_selectable(ui, color, COLORS[idx].name, is_selected) {
                    *action = Some(UiAction::SetFillColor(Some(color)));
                    ui_state.color_popover = ColorPopover::None;
                }
            }

            ui.add_space(4.0);
            widgets_vertical_separator(ui);
            ui.add_space(4.0);

            let current_fill = ui_state.fill_color.unwrap_or(Color32::TRANSPARENT);
            let (clicked, rect) = color_swatch_current(ui, current_fill, "Pick color");
            *fill_current_rect = rect;
            if clicked {
                ui_state.color_popover =
                    if ui_state.color_popover == ColorPopover::FillFull {
                        ColorPopover::None
                    } else {
                        ColorPopover::FillFull
                    };
            }
        });
    });
}

pub(super) fn render_stroke_width_section(
    ui: &mut egui::Ui,
    ui_state: &UiState,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Stroke width");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
            let widths = [
                (1.0f32, "Thin", icon!("stroke-thin.png")),
                (2.0, "Bold", icon!("stroke-bold.png")),
                (4.0, "Extra Bold", icon!("stroke-extra-bold.png")),
            ];
            for (width, name, icon) in widths {
                let is_selected = (ui_state.stroke_width - width).abs() < 0.1;
                if IconButton::new(icon, name).small().selected(is_selected).show(ui) {
                    *action = Some(UiAction::SetStrokeWidth(width));
                }
            }
        });
    });
}

pub(super) fn render_stroke_style_section(
    ui: &mut egui::Ui,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Stroke style");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
            let styles = [
                (0u8, "Solid", icon!("style-solid.png")),
                (1, "Dashed", icon!("style-dashed.png")),
                (2, "Dotted", icon!("style-dotted.png")),
            ];
            for (idx, name, icon) in styles {
                let is_selected = props.stroke_style == idx;
                if IconButton::new(icon, name).small().selected(is_selected).show(ui) && !is_selected {
                    *action = Some(UiAction::SetStrokeStyle(idx));
                }
            }
        });
    });
}

pub(super) fn render_sloppiness_section(
    ui: &mut egui::Ui,
    props: &SelectedShapeProps,
    action: &mut Option<UiAction>,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 4.0);
        widgets_section_label(ui, "Sloppiness");
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
            let levels = [
                (0u8, "Architect", icon!("sloppiness-architect.png")),
                (1, "Artist", icon!("sloppiness-artist.png")),
                (2, "Cartoonist", icon!("sloppiness-cartoonist.png")),
            ];
            for (idx, name, icon) in levels {
                let is_selected = props.sloppiness == idx;
                if IconButton::new(icon, name).small().selected(is_selected).show(ui) && !is_selected {
                    *action = Some(UiAction::SetSloppiness(idx));
                }
            }
        });
    });
}

fn color_swatch_selectable(ui: &mut egui::Ui, color: Color32, name: &str, selected: bool) -> bool {
    let (clicked, _) = ColorSwatch::new(color, name).selected(selected).show(ui);
    clicked
}

fn fill_swatch(ui: &mut egui::Ui, color: Option<Color32>, name: &str, selected: bool) -> bool {
    match color {
        Some(c) => {
            let (clicked, _) = ColorSwatch::new(c, name).selected(selected).show(ui);
            clicked
        }
        None => NoColorSwatch::new(name).selected(selected).show(ui),
    }
}
