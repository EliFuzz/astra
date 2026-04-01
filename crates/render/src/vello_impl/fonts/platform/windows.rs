pub(crate) fn find_math_font() -> Option<Vec<u8>> {
    [
        r"C:\Windows\Fonts\STIXTwoMath-Regular.otf",
        r"C:\Windows\Fonts\NotoSansMath-Regular.ttf",
    ]
    .iter()
    .find_map(|p| std::fs::read(p).ok())
}
