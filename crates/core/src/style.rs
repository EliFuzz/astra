use peniko::Color;
use serde::{Deserialize, Serialize};

use crate::color::SerializableColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Sloppiness {
    Architect = 0,
    #[default]
    Artist = 1,
    Cartoonist = 2,
    Drunk = 3,
}

impl Sloppiness {
    pub fn roughness(&self) -> f64 {
        match self {
            Sloppiness::Architect => 0.0,
            Sloppiness::Artist => 1.0,
            Sloppiness::Cartoonist => 2.0,
            Sloppiness::Drunk => 3.5,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Sloppiness::Architect => Sloppiness::Artist,
            Sloppiness::Artist => Sloppiness::Cartoonist,
            Sloppiness::Cartoonist => Sloppiness::Drunk,
            Sloppiness::Drunk => Sloppiness::Architect,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FillPattern {
    #[default]
    Solid,
    Hachure,
    ZigZag,
    CrossHatch,
    Dots,
    Dashed,
    ZigZagLine,
}

impl FillPattern {
    pub fn next(self) -> Self {
        match self {
            FillPattern::Solid => FillPattern::Hachure,
            FillPattern::Hachure => FillPattern::ZigZag,
            FillPattern::ZigZag => FillPattern::CrossHatch,
            FillPattern::CrossHatch => FillPattern::Dots,
            FillPattern::Dots => FillPattern::Dashed,
            FillPattern::Dashed => FillPattern::ZigZagLine,
            FillPattern::ZigZagLine => FillPattern::Solid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StrokeStyle {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl StrokeStyle {
    pub fn next(self) -> Self {
        match self {
            StrokeStyle::Solid => StrokeStyle::Dashed,
            StrokeStyle::Dashed => StrokeStyle::Dotted,
            StrokeStyle::Dotted => StrokeStyle::Solid,
        }
    }
}

fn default_opacity() -> f64 {
    1.0
}

fn color_with_opacity(color: Color, opacity: f64) -> Color {
    let rgba = color.to_rgba8();
    let alpha = (rgba.a as f64 * opacity) as u8;
    Color::from_rgba8(rgba.r, rgba.g, rgba.b, alpha)
}

pub fn generate_seed() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};

    static SEED_COUNTER: AtomicU32 = AtomicU32::new(1);

    let counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut x = counter.wrapping_mul(0x9E3779B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x85EBCA6B);
    x ^= x >> 13;
    x = x.wrapping_mul(0xC2B2AE35);
    x ^= x >> 16;
    x
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeStyle {
    pub stroke_color: SerializableColor,
    pub stroke_width: f64,
    pub fill_color: Option<SerializableColor>,
    #[serde(default)]
    pub fill_pattern: FillPattern,
    pub sloppiness: Sloppiness,
    #[serde(default = "generate_seed")]
    pub seed: u32,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
}

impl ShapeStyle {
    pub fn stroke(&self) -> Color {
        self.stroke_color.into()
    }

    pub fn stroke_with_opacity(&self) -> Color {
        color_with_opacity(self.stroke_color.into(), self.opacity)
    }

    pub fn fill(&self) -> Option<Color> {
        self.fill_color.map(|c| c.into())
    }

    pub fn fill_with_opacity(&self) -> Option<Color> {
        self.fill_color
            .map(|c| color_with_opacity(c.into(), self.opacity))
    }

    pub fn set_stroke(&mut self, color: Color) {
        self.stroke_color = color.into();
    }

    pub fn set_fill(&mut self, color: Option<Color>) {
        self.fill_color = color.map(|c| c.into());
    }
}

impl Default for ShapeStyle {
    fn default() -> Self {
        Self {
            stroke_color: SerializableColor::black(),
            stroke_width: 2.0,
            fill_color: None,
            fill_pattern: FillPattern::default(),
            sloppiness: Sloppiness::default(),
            seed: generate_seed(),
            opacity: 1.0,
        }
    }
}
