use egui::{
    Color32, CornerRadius, CursorIcon, Rect, Sense, Stroke, StrokeKind, Ui, Vec2, vec2,
};

use crate::{sizing, theme};

#[derive(Clone)]
pub struct ColorSwatchStyle {
    pub size: Vec2,
    pub circular: bool,
    pub selection_style: SelectionStyle,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SelectionStyle {
    None,
    InnerRing,
    OuterBorder,
}

impl Default for ColorSwatchStyle {
    fn default() -> Self {
        Self {
            size: vec2(sizing::SMALL, sizing::SMALL),
            circular: true,
            selection_style: SelectionStyle::InnerRing,
        }
    }
}

impl ColorSwatchStyle {
    pub fn small() -> Self {
        Self::default()
    }

    pub fn grid() -> Self {
        Self {
            size: vec2(16.0, 16.0),
            circular: true,
            selection_style: SelectionStyle::InnerRing,
        }
    }

    pub fn large() -> Self {
        Self {
            size: vec2(28.0, 28.0),
            circular: true,
            selection_style: SelectionStyle::InnerRing,
        }
    }
}

pub struct ColorSwatch<'a> {
    color: Color32,
    tooltip: &'a str,
    selected: bool,
    style: ColorSwatchStyle,
}

impl<'a> ColorSwatch<'a> {
    pub fn new(color: Color32, tooltip: &'a str) -> Self {
        Self {
            color,
            tooltip,
            selected: false,
            style: ColorSwatchStyle::default(),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: ColorSwatchStyle) -> Self {
        self.style = style;
        self
    }

    pub fn grid(mut self) -> Self {
        self.style = ColorSwatchStyle::grid();
        self
    }

    pub fn show(self, ui: &mut Ui) -> (bool, Rect) {
        let (rect, response) = ui.allocate_exact_size(self.style.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = rect.width().min(rect.height()) / 2.0;

            if self.style.circular {
                ui.painter().circle_filled(center, radius, self.color);

                if self.selected && self.style.selection_style == SelectionStyle::InnerRing {
                    ui.painter().circle_stroke(
                        center,
                        radius - 3.0,
                        Stroke::new(2.0, theme::accent()),
                    );
                } else if self.selected && self.style.selection_style == SelectionStyle::OuterBorder
                {
                    ui.painter()
                        .circle_stroke(center, radius, Stroke::new(2.0, theme::accent()));
                }
            } else {
                ui.painter().rect_filled(
                    rect,
                    CornerRadius::same(sizing::CORNER_RADIUS),
                    self.color,
                );

                if self.selected {
                    ui.painter().rect_stroke(
                        rect,
                        CornerRadius::same(sizing::CORNER_RADIUS),
                        Stroke::new(2.0, theme::accent()),
                        StrokeKind::Inside,
                    );
                }
            }
        }

        let clicked = response.clicked();
        response
            .on_hover_text(self.tooltip)
            .on_hover_cursor(CursorIcon::PointingHand);
        (clicked, rect)
    }
}
