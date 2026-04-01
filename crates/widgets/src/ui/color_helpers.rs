use crate::ColorSwatchWithWheel;
use egui::{Color32, Rect};

pub fn color_swatch_current(ui: &mut egui::Ui, color: Color32, tooltip: &str) -> (bool, Rect) {
    ColorSwatchWithWheel::new(color, tooltip).show(ui)
}
