use egui::{Align2, Color32, CursorIcon, Pos2, Sense, Ui, vec2};

use crate::{sizing, theme};

pub struct ToggleButton<'a> {
    label: &'a str,
    selected: bool,
    min_width: Option<f32>,
    height: f32,
    font_size: f32,
}

impl<'a> ToggleButton<'a> {
    pub fn new(label: &'a str, selected: bool) -> Self {
        Self {
            label,
            selected,
            min_width: None,
            height: 22.0,
            font_size: 10.5,
        }
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let font_id = egui::FontId::proportional(self.font_size);
        let galley = ui.painter().layout_no_wrap(
            self.label.to_string(),
            font_id.clone(),
            Color32::PLACEHOLDER,
        );
        let text_width = galley.size().x;
        let width = self
            .min_width
            .unwrap_or(text_width + 16.0)
            .max(text_width + 16.0);
        let size = vec2(width, self.height);

        let (rect, response) = ui.allocate_exact_size(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if self.selected {
                theme::selected_bg()
            } else if response.hovered() {
                theme::hover_bg()
            } else {
                theme::button_inactive_bg()
            };

            let text_color = if self.selected {
                theme::accent()
            } else {
                theme::icon_color()
            };

            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(sizing::CORNER_RADIUS),
                bg_color,
            );

            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                self.label,
                font_id,
                text_color,
            );
        }

        let clicked = response.clicked();
        response.on_hover_cursor(CursorIcon::PointingHand);
        clicked
    }
}

pub struct TextButton<'a> {
    label: &'a str,
    shortcut: Option<&'a str>,
}

impl<'a> TextButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            shortcut: None,
        }
    }

    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let size = vec2(0.0, 24.0);
        let (rect, response) = ui.allocate_at_least(size, Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if response.hovered() {
                theme::hover_bg()
            } else {
                Color32::TRANSPARENT
            };

            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(sizing::CORNER_RADIUS),
                bg_color,
            );

            ui.painter().text(
                Pos2::new(rect.left() + 8.0, rect.center().y),
                Align2::LEFT_CENTER,
                self.label,
                egui::FontId::proportional(12.0),
                theme::text(),
            );

            if let Some(shortcut) = self.shortcut {
                ui.painter().text(
                    Pos2::new(rect.right() - 8.0, rect.center().y),
                    Align2::RIGHT_CENTER,
                    shortcut,
                    egui::FontId::proportional(11.0),
                    theme::text_muted(),
                );
            }
        }

        let clicked = response.clicked();
        response.on_hover_cursor(CursorIcon::PointingHand);
        clicked
    }
}
