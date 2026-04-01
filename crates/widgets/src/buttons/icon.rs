use egui::{Color32, CornerRadius, CursorIcon, Image, ImageSource, Rect, Sense, Ui, Vec2, vec2};

use crate::{sizing, theme};

#[derive(Clone)]
pub struct IconButtonStyle {
    pub size: Vec2,
    pub icon_size: Vec2,
    pub corner_radius: u8,
    pub solid_selected: bool,
}

impl Default for IconButtonStyle {
    fn default() -> Self {
        Self {
            size: vec2(sizing::MEDIUM, sizing::MEDIUM),
            icon_size: vec2(16.0, 16.0),
            corner_radius: sizing::CORNER_RADIUS,
            solid_selected: true,
        }
    }
}

impl IconButtonStyle {
    pub fn small() -> Self {
        Self {
            size: vec2(22.0, 22.0),
            icon_size: vec2(14.0, 14.0),
            corner_radius: sizing::CORNER_RADIUS,
            solid_selected: true,
        }
    }

    pub fn tool() -> Self {
        Self {
            size: vec2(28.0, 28.0),
            icon_size: vec2(16.0, 16.0),
            corner_radius: 6,
            solid_selected: true,
        }
    }

    pub fn large() -> Self {
        Self {
            size: vec2(sizing::LARGE, sizing::LARGE),
            icon_size: vec2(20.0, 20.0),
            ..Default::default()
        }
    }
}

pub struct IconButton<'a> {
    icon: ImageSource<'a>,
    tooltip: &'a str,
    shortcut: Option<&'a str>,
    selected: bool,
    style: IconButtonStyle,
}

impl<'a> IconButton<'a> {
    pub fn new(icon: ImageSource<'a>, tooltip: &'a str) -> Self {
        Self {
            icon,
            tooltip,
            shortcut: None,
            selected: false,
            style: IconButtonStyle::default(),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn small(mut self) -> Self {
        self.style = IconButtonStyle::small();
        self
    }

    pub fn tool(mut self) -> Self {
        self.style = IconButtonStyle::tool();
        self
    }

    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let (rect, response) = ui.allocate_exact_size(self.style.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let bg_color = if self.selected && self.style.solid_selected {
                theme::selected_bg()
            } else if response.hovered() {
                theme::hover_bg()
            } else {
                Color32::TRANSPARENT
            };

            ui.painter()
                .rect_filled(rect, CornerRadius::same(self.style.corner_radius), bg_color);

            let icon_tint = if self.selected {
                Some(theme::accent())
            } else if response.hovered() {
                Some(theme::icon_hover_color())
            } else {
                Some(theme::icon_color())
            };

            let icon_rect = Rect::from_center_size(rect.center(), self.style.icon_size);
            let mut image = Image::new(self.icon).fit_to_exact_size(self.style.icon_size);
            if let Some(tint) = icon_tint {
                image = image.tint(tint);
            }
            image.paint_at(ui, icon_rect);
        }

        let clicked = response.clicked();
        if let Some(shortcut) = self.shortcut {
            response.clone().on_hover_ui(|ui| {
                ui.horizontal(|ui| {
                    ui.label(self.tooltip);
                    ui.label(
                        egui::RichText::new(format!("({})", shortcut))
                            .color(Color32::from_gray(128))
                            .small(),
                    );
                });
            });
        } else {
            response.clone().on_hover_text(self.tooltip);
        }
        response.on_hover_cursor(CursorIcon::PointingHand);
        clicked
    }
}
