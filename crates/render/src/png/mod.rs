#[cfg(feature = "vello-renderer")]
mod gpu;

#[cfg(all(not(target_arch = "wasm32"), feature = "vello-renderer"))]
mod native;

#[cfg(all(target_arch = "wasm32", feature = "vello-renderer"))]
mod wasm;

pub use astra_core::png::{PNG_SCENE_METADATA_KEY, encode_png, extract_scene_from_png};

#[cfg(feature = "vello-renderer")]
pub use gpu::{RenderBuffer, TiledExport, STRIP_HEIGHT, build_tile_scene, prepare_readback};

#[cfg(all(not(target_arch = "wasm32"), feature = "vello-renderer"))]
pub use native::{export_scene_to_png_file, render_scene_to_png};

#[cfg(all(target_arch = "wasm32", feature = "vello-renderer"))]
pub use wasm::start_png_readback;
