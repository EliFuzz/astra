pub(crate) fn find_math_font() -> Option<Vec<u8>> {
    [
        "/System/Library/Fonts/Supplemental/STIX Two Math.otf",
        "/Library/Fonts/STIXTwoMath-Regular.otf",
        "/System/Library/Fonts/Supplemental/NotoSansMath-Regular.ttf",
    ]
    .iter()
    .find_map(|p| std::fs::read(p).ok())
}
