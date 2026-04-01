mod color_swatch;
mod no_color;

pub use color_swatch::{ColorSwatch, ColorSwatchStyle, SelectionStyle};
pub use no_color::{NoColorSwatch, colors_match, parse_css_color};
