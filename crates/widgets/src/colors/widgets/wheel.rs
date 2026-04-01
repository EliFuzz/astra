use egui::{Color32, CursorIcon, Pos2, Rect, Sense, Stroke, Ui, Vec2, vec2};

use crate::sizing;

pub struct ColorSwatchWithWheel<'a> {
    color: Color32,
    tooltip: &'a str,
    size: Vec2,
}

impl<'a> ColorSwatchWithWheel<'a> {
    pub fn new(color: Color32, tooltip: &'a str) -> Self {
        Self {
            color,
            tooltip,
            size: vec2(sizing::SMALL, sizing::SMALL),
        }
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn show(self, ui: &mut Ui) -> (bool, Rect) {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::click());

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let outer_radius = rect.width().min(rect.height()) / 2.0;
            let ring_width = 3.0;
            let inner_radius = outer_radius - ring_width;

            let num_segments = 32;
            for i in 0..num_segments {
                let angle1 = (i as f32 / num_segments as f32) * std::f32::consts::TAU;
                let angle2 = ((i + 1) as f32 / num_segments as f32) * std::f32::consts::TAU;

                let hue = i as f32 / num_segments as f32;
                let hue_color = hue_to_rgb(hue);

                let p1 = Pos2::new(
                    center.x + outer_radius * angle1.cos(),
                    center.y + outer_radius * angle1.sin(),
                );
                let p2 = Pos2::new(
                    center.x + outer_radius * angle2.cos(),
                    center.y + outer_radius * angle2.sin(),
                );
                let p3 = Pos2::new(
                    center.x + inner_radius * angle2.cos(),
                    center.y + inner_radius * angle2.sin(),
                );
                let p4 = Pos2::new(
                    center.x + inner_radius * angle1.cos(),
                    center.y + inner_radius * angle1.sin(),
                );

                ui.painter().add(egui::Shape::convex_polygon(
                    vec![p1, p2, p3, p4],
                    hue_color,
                    Stroke::NONE,
                ));
            }

            ui.painter()
                .circle_filled(center, inner_radius, crate::theme::panel_bg());

            ui.painter()
                .circle_filled(center, inner_radius - 2.0, self.color);
        }

        let clicked = response.clicked();
        response
            .on_hover_text(self.tooltip)
            .on_hover_cursor(CursorIcon::PointingHand);
        (clicked, rect)
    }
}

pub fn hue_to_rgb(hue: f32) -> Color32 {
    let h = hue * 6.0;
    let c = 1.0_f32;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());

    let (r, g, b) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}
