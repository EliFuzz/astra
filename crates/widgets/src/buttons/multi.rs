use egui::{CornerRadius, CursorIcon, Image, ImageSource, Rect, Sense, Ui};

use super::icon::IconButtonStyle;
use crate::theme;

pub struct MultiToggleState<'a, T: Clone + PartialEq> {
    pub value: T,
    pub icon: ImageSource<'a>,
    pub tooltip: &'a str,
}

impl<'a, T: Clone + PartialEq> MultiToggleState<'a, T> {
    pub fn new(value: T, icon: ImageSource<'a>, tooltip: &'a str) -> Self {
        Self {
            value,
            icon,
            tooltip,
        }
    }
}

pub struct MultiToggle<'a, T: Clone + PartialEq> {
    states: &'a [MultiToggleState<'a, T>],
    current: &'a T,
    style: IconButtonStyle,
}

impl<'a, T: Clone + PartialEq> MultiToggle<'a, T> {
    pub fn new(states: &'a [MultiToggleState<'a, T>], current: &'a T) -> Self {
        Self {
            states,
            current,
            style: IconButtonStyle::default(),
        }
    }

    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn small(mut self) -> Self {
        self.style = IconButtonStyle::small();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<T> {
        let current_idx = self
            .states
            .iter()
            .position(|s| &s.value == self.current)
            .unwrap_or(0);

        let state = &self.states[current_idx];
        let (rect, response) = ui.allocate_exact_size(self.style.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if response.hovered() {
                theme::hover_bg()
            } else {
                egui::Color32::TRANSPARENT
            };

            ui.painter()
                .rect_filled(rect, CornerRadius::same(self.style.corner_radius), bg_color);

            let icon_rect = Rect::from_center_size(rect.center(), self.style.icon_size);
            Image::new(state.icon.clone()).paint_at(ui, icon_rect);
        }

        let clicked = response.clicked();
        response
            .on_hover_text(state.tooltip)
            .on_hover_cursor(CursorIcon::PointingHand);

        if clicked {
            let next_idx = (current_idx + 1) % self.states.len();
            Some(self.states[next_idx].value.clone())
        } else {
            None
        }
    }
}
