use crate::vello_impl::fonts::LoadedFonts;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
use super::fallback::find_math_font;
#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
use super::linux::find_math_font;
#[cfg(target_os = "macos")]
use super::macos::find_math_font;
#[cfg(target_os = "windows")]
use super::windows::find_math_font;

pub(crate) fn load() {
    let xits_math = find_math_font().unwrap_or_else(|| {
        log::info!("Math font not found on system; LaTeX rendering unavailable");
        Vec::new()
    });

    crate::vello_impl::fonts::set(LoadedFonts {
        registered: Vec::new(),
        xits_math,
        primary_math_face: Vec::new(),
    });
}
