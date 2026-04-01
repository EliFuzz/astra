use egui::{Color32, CursorIcon, Pos2, Sense, Stroke, Ui, Vec2, vec2};

use crate::sizing;

pub struct NoColorSwatch<'a> {
    tooltip: &'a str,
    selected: bool,
    size: Vec2,
}

impl<'a> NoColorSwatch<'a> {
    pub fn new(tooltip: &'a str) -> Self {
        Self {
            tooltip,
            selected: false,
            size: vec2(sizing::SMALL, sizing::SMALL),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn grid(mut self) -> Self {
        self.size = vec2(16.0, 16.0);
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = rect.width().min(rect.height()) / 2.0;

            let bg = if crate::theme::is_dark() {
                Color32::from_gray(40)
            } else {
                Color32::WHITE
            };
            ui.painter().circle_filled(center, radius, bg);
            ui.painter()
                .circle_stroke(center, radius, Stroke::new(1.0, crate::theme::border()));

            let offset = radius * 0.6;
            ui.painter().line_segment(
                [
                    Pos2::new(center.x - offset, center.y + offset),
                    Pos2::new(center.x + offset, center.y - offset),
                ],
                Stroke::new(2.0, Color32::from_rgb(239, 68, 68)),
            );

            if self.selected {
                ui.painter().circle_stroke(
                    center,
                    radius - 3.0,
                    Stroke::new(2.0, crate::theme::accent()),
                );
            }
        }

        let clicked = response.clicked();
        response
            .on_hover_text(self.tooltip)
            .on_hover_cursor(CursorIcon::PointingHand);
        clicked
    }
}

pub fn colors_match(a: Color32, b: Color32) -> bool {
    a.r() == b.r() && a.g() == b.g() && a.b() == b.b()
}

pub fn parse_css_color(color: &str) -> Color32 {
    if color.starts_with('#') && color.len() == 7 {
        let r = u8::from_str_radix(&color[1..3], 16).unwrap_or(128);
        let g = u8::from_str_radix(&color[3..5], 16).unwrap_or(128);
        let b = u8::from_str_radix(&color[5..7], 16).unwrap_or(128);
        Color32::from_rgb(r, g, b)
    } else {
        Color32::from_rgb(128, 128, 128)
    }
}
