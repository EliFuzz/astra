use parley::FontContext;
use std::sync::{Arc, OnceLock};

static FONTS: OnceLock<LoadedFonts> = OnceLock::new();

pub(crate) struct LoadedFonts {
    pub registered: Vec<Vec<u8>>,
    pub xits_math: Vec<u8>,
    pub primary_math_face: Vec<u8>,
}

pub(crate) fn set(fonts: LoadedFonts) {
    let _ = FONTS.set(fonts);
}

pub(crate) fn load_and_register(font_cx: &mut FontContext) {
    super::platform::load();
    register_all(font_cx);
}

fn register_all(font_cx: &mut FontContext) {
    let Some(fonts) = FONTS.get() else { return };
    for data in &fonts.registered {
        font_cx
            .collection
            .register_fonts(vello::peniko::Blob::new(Arc::new(data.clone())), None);
    }
}

pub(crate) fn primary_math_face() -> &'static [u8] {
    FONTS
        .get()
        .map(|f| f.primary_math_face.as_slice())
        .unwrap_or(&[])
}

pub(crate) fn xits_math() -> &'static [u8] {
    FONTS.get().map(|f| f.xits_math.as_slice()).unwrap_or(&[])
}
