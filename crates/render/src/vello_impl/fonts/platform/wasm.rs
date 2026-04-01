use crate::vello_impl::fonts::LoadedFonts;
use std::cell::RefCell;

thread_local! {
    static CANVAS_FONTS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    static MATH_FONT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn decode_if_woff2(data: Vec<u8>) -> Vec<u8> {
    if data.starts_with(b"wOF2") {
        wuff::decompress_woff2(&data).unwrap_or(data)
    } else {
        data
    }
}

pub fn register_canvas_font(data: Vec<u8>) {
    CANVAS_FONTS.with(|f| f.borrow_mut().push(decode_if_woff2(data)));
}

pub fn register_math_font(data: Vec<u8>) {
    MATH_FONT.with(|f| *f.borrow_mut() = decode_if_woff2(data));
}

pub fn commit_fonts() {
    let registered = CANVAS_FONTS.with(|f| f.borrow().clone());
    let xits_math = MATH_FONT.with(|f| f.borrow().clone());
    let primary_math_face = registered.first().cloned().unwrap_or_default();
    crate::vello_impl::fonts::set(LoadedFonts {
        registered,
        xits_math,
        primary_math_face,
    });
}
