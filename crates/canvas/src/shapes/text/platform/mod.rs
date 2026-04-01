#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::{handwriting_font_stack, sans_font_stack};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{handwriting_font_stack, sans_font_stack};

#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
mod linux;
#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
pub use linux::{handwriting_font_stack, sans_font_stack};

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::{handwriting_font_stack, sans_font_stack};

#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_arch = "wasm32"
)))]
mod fallback;
#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_arch = "wasm32"
)))]
pub use fallback::{handwriting_font_stack, sans_font_stack};
