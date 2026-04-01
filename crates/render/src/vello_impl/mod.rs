mod cache;
mod effects;
pub(crate) mod fonts;
mod grid;
mod shapes;
mod text;
mod vello_renderer;

pub use cache::PngRenderResult;
pub use vello_renderer::VelloRenderer;

#[cfg(target_arch = "wasm32")]
pub use fonts::{commit_fonts, register_canvas_font, register_math_font};

#[cfg(test)]
mod tests;
