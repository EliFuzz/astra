mod platform;
mod registry;

pub(crate) use registry::{load_and_register, primary_math_face, set, xits_math, LoadedFonts};

#[cfg(target_arch = "wasm32")]
pub use platform::wasm::{commit_fonts, register_canvas_font, register_math_font};
