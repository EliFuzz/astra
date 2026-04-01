use astra_canvas::shapes::ShapeTrait;
use std::hash::{Hash, Hasher};

pub(crate) fn text_layout_cache_key(text: &astra_canvas::shapes::Text) -> (String, u64) {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.content.hash(&mut hasher);
    (text.font_family as u8).hash(&mut hasher);
    (text.font_weight as u8).hash(&mut hasher);
    text.font_size.to_bits().hash(&mut hasher);
    text.char_colors.len().hash(&mut hasher);
    for c in &text.char_colors {
        c.is_some().hash(&mut hasher);
        if let Some(color) = c {
            color.r.hash(&mut hasher);
            color.g.hash(&mut hasher);
            color.b.hash(&mut hasher);
        }
    }
    text.style.stroke_color.r.hash(&mut hasher);
    text.style.stroke_color.g.hash(&mut hasher);
    text.style.stroke_color.b.hash(&mut hasher);
    text.style.opacity.to_bits().hash(&mut hasher);
    text.text_align.hash(&mut hasher);
    (text.id().to_string(), hasher.finish())
}
