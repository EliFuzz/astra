#[derive(Debug)]
pub(crate) enum TextSegment<'a> {
    Plain(&'a str),
    LaTeX(&'a str),
}

pub(crate) fn parse_segments(line: &str) -> Vec<TextSegment<'_>> {
    let mut segments = Vec::new();
    let mut rest = line;
    while !rest.is_empty() {
        if let Some(start) = rest.find(r"\$") {
            if start > 0 {
                segments.push(TextSegment::Plain(&rest[..start]));
            }
            rest = &rest[start + 2..];
            if let Some(end) = rest.find(r"\$") {
                segments.push(TextSegment::LaTeX(&rest[..end]));
                rest = &rest[end + 2..];
            } else {
                segments.push(TextSegment::Plain(r"\$"));
                segments.push(TextSegment::Plain(rest));
                break;
            }
        } else {
            segments.push(TextSegment::Plain(rest));
            break;
        }
    }
    segments
}
