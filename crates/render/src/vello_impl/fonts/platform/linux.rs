pub(crate) fn find_math_font() -> Option<Vec<u8>> {
    [
        "/usr/share/fonts/opentype/stix/STIXTwoMath-Regular.otf",
        "/usr/share/fonts/truetype/stix/STIXTwoMath-Regular.otf",
        "/usr/local/share/fonts/STIXTwoMath-Regular.otf",
        "/usr/share/fonts/NotoSansMath-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSansMath-Regular.ttf",
    ]
    .iter()
    .find_map(|p| std::fs::read(p).ok())
}
