#[cfg(not(target_arch = "wasm32"))]
pub(super) mod native;

#[cfg(target_arch = "wasm32")]
pub(crate) mod wasm;

#[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
pub(super) mod macos;

#[cfg(all(not(target_arch = "wasm32"), target_os = "windows"))]
pub(super) mod windows;

#[cfg(all(not(target_arch = "wasm32"), target_os = "linux"))]
pub(super) mod linux;

#[cfg(all(
    not(target_arch = "wasm32"),
    not(any(target_os = "macos", target_os = "windows", target_os = "linux"))
))]
pub(super) mod fallback;

pub(super) fn load() {
    #[cfg(not(target_arch = "wasm32"))]
    native::load();
}
