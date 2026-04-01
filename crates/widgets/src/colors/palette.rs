use super::color_data::COLORS;
pub use super::color_data::Color;

pub const SHADE_LABELS: [&str; 12] = [
    "25", "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
];

pub struct Palette;

impl Palette {
    pub fn all() -> &'static [Color] {
        COLORS
    }

    pub fn by_name(name: &str) -> Option<&'static Color> {
        COLORS.iter().find(|c| c.name == name)
    }

    pub fn quick_colors() -> &'static [usize] {
        &[10, 2, 4, 11, 13, 16, 17]
    }
}
