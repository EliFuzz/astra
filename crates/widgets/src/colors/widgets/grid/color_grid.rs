use egui::{Color32, Pos2, Rect, vec2};

use super::quick::{QuickColor, quick_color_cell};
use super::super::swatch::{ColorSwatch, colors_match};
use crate::colors::{COLORS, SHADE_LABELS};
use crate::theme;

pub struct ColorGrid<'a> {
    current_color: Color32,
    title: &'a str,
    shade_indices: &'a [usize],
    position: ColorGridPosition,
}

#[derive(Clone, Copy, Default)]
pub enum ColorGridPosition {
    #[default]
    Below,
    Above,
}

impl<'a> ColorGrid<'a> {
    pub fn new(current_color: Color32, title: &'a str) -> Self {
        Self {
            current_color,
            title,
            shade_indices: &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            position: ColorGridPosition::Below,
        }
    }

    pub fn shades(mut self, indices: &'a [usize]) -> Self {
        self.shade_indices = indices;
        self
    }

    pub fn above(mut self) -> Self {
        self.position = ColorGridPosition::Above;
        self
    }

    pub fn below(mut self) -> Self {
        self.position = ColorGridPosition::Below;
        self
    }

    pub fn show(self, ctx: &egui::Context, anchor_rect: Rect) -> Option<Color32> {
        let mut selected = None;

        let grid_height = 220.0;
        let pos = match self.position {
            ColorGridPosition::Below => {
                Pos2::new(anchor_rect.left() - 100.0, anchor_rect.bottom() + 8.0)
            }
            ColorGridPosition::Above => Pos2::new(
                anchor_rect.left() - 50.0,
                anchor_rect.top() - grid_height - 8.0,
            ),
        };

        let quick_colors: [QuickColor; 9] = [
            QuickColor::None,
            QuickColor::Black,
            QuickColor::White,
            QuickColor::Palette(10),
            QuickColor::Palette(0),
            QuickColor::Palette(6),
            QuickColor::Palette(2),
            QuickColor::Palette(13),
            QuickColor::Palette(17),
        ];
        let quick_tooltips = [
            "None", "Black", "White", "Blue", "Red", "Emerald", "Amber", "Purple", "Slate",
        ];

        egui::Area::new(egui::Id::new("color_grid"))
            .fixed_pos(pos)
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                crate::panel_frame().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 4.0);

                        ui.label(
                            egui::RichText::new(self.title)
                                .size(12.0)
                                .color(theme::text_muted()),
                        );

                        ui.add_space(4.0);

                        for (row_idx, &shade_idx) in self.shade_indices.iter().enumerate() {
                            if shade_idx == 2 || shade_idx == 6 {
                                ui.add_space(4.0);
                            }
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);

                                for palette_color in COLORS.iter() {
                                    let color = palette_color.shades[shade_idx];
                                    let is_selected = colors_match(self.current_color, color);
                                    let tooltip =
                                        format!("{} {}", palette_color.name, SHADE_LABELS[shade_idx]);
                                    let (clicked, _) = ColorSwatch::new(color, &tooltip)
                                        .selected(is_selected)
                                        .grid()
                                        .show(ui);
                                    if clicked {
                                        selected = Some(color);
                                    }
                                }

                                if row_idx < quick_colors.len() {
                                    ui.add_space(4.0);
                                    if let Some(c) = quick_color_cell(
                                        ui,
                                        self.current_color,
                                        quick_colors[row_idx],
                                        quick_tooltips[row_idx],
                                    ) {
                                        selected = Some(c);
                                    }
                                }
                            });
                            if shade_idx == 6 {
                                ui.add_space(4.0);
                            }
                        }
                    });
                });
            });

        selected
    }
}
