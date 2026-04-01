use crate::theme;
use astra_render::GridStyle;
use egui::{CornerRadius, Pos2, Rect, Stroke, Vec2};

pub(super) fn grid_style_button(ui: &mut egui::Ui, style: GridStyle, tooltip: &str) -> bool {
    let size = Vec2::new(24.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let icon_color = if response.hovered() {
            theme::icon_hover_color()
        } else {
            theme::icon_color()
        };

        let center = rect.center();
        let grid_size = 5.0;

        match style {
            GridStyle::None => {
                let sq_size = 12.0;
                let sq = Rect::from_center_size(center, Vec2::splat(sq_size));
                ui.painter().rect_stroke(
                    sq,
                    CornerRadius::same(2),
                    Stroke::new(1.5, icon_color),
                    egui::StrokeKind::Inside,
                );
                ui.painter().line_segment(
                    [sq.left_bottom(), sq.right_top()],
                    Stroke::new(1.0, theme::text_muted()),
                );
            }
            GridStyle::Lines => {
                for i in 0..3 {
                    let offset = (i as f32 - 1.0) * grid_size;
                    ui.painter().line_segment(
                        [
                            Pos2::new(center.x + offset, center.y - grid_size * 1.5),
                            Pos2::new(center.x + offset, center.y + grid_size * 1.5),
                        ],
                        Stroke::new(1.0, icon_color),
                    );
                    ui.painter().line_segment(
                        [
                            Pos2::new(center.x - grid_size * 1.5, center.y + offset),
                            Pos2::new(center.x + grid_size * 1.5, center.y + offset),
                        ],
                        Stroke::new(1.0, icon_color),
                    );
                }
            }
            GridStyle::HorizontalLines => {
                for i in 0..3 {
                    let offset = (i as f32 - 1.0) * grid_size;
                    ui.painter().line_segment(
                        [
                            Pos2::new(center.x - grid_size * 1.5, center.y + offset),
                            Pos2::new(center.x + grid_size * 1.5, center.y + offset),
                        ],
                        Stroke::new(1.0, icon_color),
                    );
                }
            }
            GridStyle::CrossPlus => {
                let cross_size = 2.0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        let cx = center.x + (i as f32) * grid_size;
                        let cy = center.y + (j as f32) * grid_size;
                        ui.painter().line_segment(
                            [
                                Pos2::new(cx - cross_size, cy),
                                Pos2::new(cx + cross_size, cy),
                            ],
                            Stroke::new(1.0, icon_color),
                        );
                        ui.painter().line_segment(
                            [
                                Pos2::new(cx, cy - cross_size),
                                Pos2::new(cx, cy + cross_size),
                            ],
                            Stroke::new(1.0, icon_color),
                        );
                    }
                }
            }
            GridStyle::Dots => {
                let dot_radius = 1.5;
                for i in -1..=1 {
                    for j in -1..=1 {
                        let cx = center.x + (i as f32) * grid_size;
                        let cy = center.y + (j as f32) * grid_size;
                        ui.painter()
                            .circle_filled(Pos2::new(cx, cy), dot_radius, icon_color);
                    }
                }
            }
        }
    }

    let clicked = response.clicked();
    response.on_hover_text(tooltip);
    clicked
}
