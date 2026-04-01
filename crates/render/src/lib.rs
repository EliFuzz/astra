pub mod png;
mod renderer;
pub mod text_editor;

#[cfg(feature = "vello-renderer")]
mod vello_impl;

#[cfg(feature = "vello-renderer")]
pub mod rex_backend;

pub use png::{PNG_SCENE_METADATA_KEY, encode_png, extract_scene_from_png};
pub use renderer::{
    AngleSnapInfo, GridStyle, RenderContext, Renderer, RendererError, RotationInfo,
};
pub use text_editor::{TextEditResult, TextEditState, TextKey, TextModifiers};

#[cfg(feature = "vello-renderer")]
pub use png::{RenderBuffer, TiledExport, STRIP_HEIGHT, build_tile_scene, prepare_readback};

#[cfg(all(not(target_arch = "wasm32"), feature = "vello-renderer"))]
pub use png::{export_scene_to_png_file, render_scene_to_png};

#[cfg(all(target_arch = "wasm32", feature = "vello-renderer"))]
pub use png::start_png_readback;

#[cfg(all(target_arch = "wasm32", feature = "vello-renderer"))]
pub use vello_impl::{commit_fonts, register_canvas_font, register_math_font};

#[cfg(feature = "vello-renderer")]
pub use vello_impl::{PngRenderResult, VelloRenderer};
