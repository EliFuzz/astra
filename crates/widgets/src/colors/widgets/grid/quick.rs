use egui::{Color32, CursorIcon, Sense, Ui, vec2};

use super::super::swatch::{ColorSwatch, NoColorSwatch, colors_match};
use crate::colors::COLORS;

#[derive(Clone, Copy)]
pub(super) enum QuickColor {
    None,
    Black,
    White,
    Palette(usize),
}

pub(super) fn quick_color_cell(
    ui: &mut Ui,
    current_color: Color32,
    quick: QuickColor,
    tooltip: &str,
) -> Option<Color32> {
    match quick {
        QuickColor::None => {
            let is_selected = current_color.a() == 0;
            if NoColorSwatch::new(tooltip)
                .selected(is_selected)
                .grid()
                .show(ui)
            {
                return Some(Color32::TRANSPARENT);
            }
        }
        QuickColor::Black => {
            let is_selected = colors_match(current_color, Color32::BLACK);
            let (clicked, _) =
                ColorSwatch::new(Color32::BLACK, tooltip)
                    .selected(is_selected)
                    .grid()
                    .show(ui);
            if clicked {
                return Some(Color32::BLACK);
            }
        }
        QuickColor::White => {
            let size = vec2(16.0, 16.0);
            let (rect, response) = ui.allocate_exact_size(size, Sense::click());
            if ui.is_rect_visible(rect) {
                let center = rect.center();
                let radius = 8.0;
                ui.painter()
                    .circle_filled(center, radius, Color32::WHITE);
                ui.painter().circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(0.5, crate::theme::separator_color()),
                );
                let is_selected = colors_match(current_color, Color32::WHITE);
                if is_selected {
                    ui.painter().circle_stroke(
                        center,
                        radius - 3.0,
                        egui::Stroke::new(2.0, crate::theme::accent()),
                    );
                }
            }
            let clicked = response.clicked();
            response
                .on_hover_text(tooltip)
                .on_hover_cursor(CursorIcon::PointingHand);
            if clicked {
                return Some(Color32::WHITE);
            }
        }
        QuickColor::Palette(idx) => {
            let color = COLORS[idx].shades[6];
            let is_selected = colors_match(current_color, color);
            let (clicked, _) = ColorSwatch::new(color, tooltip)
                .selected(is_selected)
                .grid()
                .show(ui);
            if clicked {
                return Some(color);
            }
        }
    }
    None
}
