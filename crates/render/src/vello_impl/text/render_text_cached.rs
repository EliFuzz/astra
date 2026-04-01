use super::super::cache::CachedTextLayout;
use crate::vello_impl::VelloRenderer;
use kurbo::Affine;
use peniko::Fill;
use vello::Scene;

impl VelloRenderer {
    pub(crate) fn draw_cached_text_layout(
        scene: &mut Scene,
        text: &astra_canvas::shapes::Text,
        transform: Affine,
        cached: &CachedTextLayout,
    ) {
        text.set_cached_size(cached.width, cached.height);
        let text_transform = transform * Affine::translate((text.position.x, text.position.y));

        for (font_data, font_size, brush, glyphs, skew) in &cached.glyph_runs {
            let glyph_xform = skew.map(|angle| Affine::skew(angle, 0.0));
            scene
                .draw_glyphs(font_data)
                .brush(brush)
                .hint(true)
                .transform(text_transform)
                .glyph_transform(glyph_xform)
                .font_size(*font_size)
                .draw(Fill::NonZero, glyphs.iter().cloned());
        }
    }
}
