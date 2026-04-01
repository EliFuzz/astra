use super::render_text_cache_key::text_layout_cache_key;
use super::super::cache::{CachedTextLayout, GlyphRun};
use crate::vello_impl::VelloRenderer;
use kurbo::{Affine, Point, Rect, Stroke};
use parley::layout::PositionedLayoutItem;
use peniko::{Brush, Color, Fill};

impl VelloRenderer {
    pub(crate) fn render_text(&mut self, text: &astra_canvas::shapes::Text, transform: Affine) {
        use parley::StyleProperty;

        if text.content.is_empty() {
            let cursor_height = text.font_size * 1.2;
            let cursor = kurbo::Line::new(
                Point::new(text.position.x, text.position.y),
                Point::new(text.position.x, text.position.y + cursor_height),
            );
            let stroke = Stroke::new(2.0);
            self.scene.stroke(
                &stroke,
                transform,
                Color::from_rgba8(100, 100, 100, 200),
                None,
                &cursor,
            );
            return;
        }

        if text.content.contains(r"\$") {
            self.render_text_mixed(text, transform);
            return;
        }

        use astra_canvas::shapes::FontWeight;

        let cache_key = text_layout_cache_key(text);

        self.text_generation = self.text_generation.wrapping_add(1);
        let touch = self.text_generation;
        if let Some(layout) = self.text_cache.get_mut(&cache_key) {
            layout.access_gen = touch;
        }
        if let Some(layout) = self.text_cache.get(&cache_key) {
            let scene = &mut self.scene;
            Self::draw_cached_text_layout(scene, text, transform, layout);
            return;
        }

        let style = &text.style;
        let brush = Brush::Solid(style.stroke_with_opacity());
        let font_size = text.font_size as f32;

        let font_stack = text.font_family.system_font_stack();
        let parley_weight = match text.font_weight {
            FontWeight::Light => parley::FontWeight::new(300.0),
            FontWeight::Regular => parley::FontWeight::NORMAL,
            FontWeight::Heavy => parley::FontWeight::BOLD,
        };

        let mut builder =
            self.layout_cx
                .ranged_builder(&mut self.font_cx, &text.content, 1.0, false);
        builder.push_default(StyleProperty::FontSize(font_size));
        builder.push_default(StyleProperty::Brush(brush.clone()));
        builder.push_default(StyleProperty::FontWeight(parley_weight));
        builder.push_default(StyleProperty::FontStack(parley::FontStack::Source(
            font_stack.into(),
        )));

        let mut byte_offset = 0;
        for (char_idx, ch) in text.content.chars().enumerate() {
            if let Some(Some(color)) = text.char_colors.get(char_idx) {
                let color: peniko::Color = (*color).into();
                let span_brush = Brush::Solid(color);
                let char_len = ch.len_utf8();
                builder.push(
                    StyleProperty::Brush(span_brush),
                    byte_offset..byte_offset + char_len,
                );
            }
            byte_offset += ch.len_utf8();
        }

        let mut layout = builder.build(&text.content);
        layout.break_all_lines(None);
        let alignment = match text.text_align {
            1 => parley::Alignment::Center,
            2 => parley::Alignment::End,
            _ => parley::Alignment::Start,
        };
        layout.align(
            None,
            alignment,
            parley::AlignmentOptions::default(),
        );

        let layout_width = layout.width() as f64;
        let layout_height = layout.height() as f64;
        text.set_cached_size(layout_width, layout_height);

        let text_transform = transform * Affine::translate((text.position.x, text.position.y));

        let mut cached_runs: Vec<GlyphRun> = Vec::new();
        let mut glyph_count = 0;

        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let run_font_size = run.font_size();
                let synthesis = run.synthesis();
                let skew_angle = synthesis
                    .skew()
                    .map(|angle| angle.to_radians().tan() as f64);
                let glyph_xform = skew_angle.map(|angle| Affine::skew(angle, 0.0));
                let run_brush = glyph_run.style().brush.clone();

                let glyphs: Vec<vello::Glyph> = glyph_run
                    .glyphs()
                    .map(|glyph| {
                        let gx = x + glyph.x;
                        let gy = y - glyph.y;
                        x += glyph.advance;
                        glyph_count += 1;
                        vello::Glyph {
                            id: glyph.id,
                            x: gx,
                            y: gy,
                        }
                    })
                    .collect();

                if !glyphs.is_empty() {
                    self.scene
                        .draw_glyphs(font)
                        .brush(&run_brush)
                        .hint(true)
                        .transform(text_transform)
                        .glyph_transform(glyph_xform)
                        .font_size(run_font_size)
                        .normalized_coords(run.normalized_coords())
                        .draw(Fill::NonZero, glyphs.iter().cloned());

                    cached_runs.push((font.clone(), run_font_size, run_brush, glyphs, skew_angle));
                }
            }
        }

        self.text_cache.insert(
            cache_key,
            CachedTextLayout {
                glyph_runs: cached_runs,
                width: layout_width,
                height: layout_height,
                access_gen: touch,
            },
        );

        while self.text_cache.len() > 500 {
            let remove_key = self
                .text_cache
                .iter()
                .min_by_key(|(_, layout)| layout.access_gen)
                .map(|(k, _)| k.clone());
            match remove_key {
                Some(k) => {
                    self.text_cache.remove(&k);
                }
                None => break,
            }
        }

        if glyph_count == 0 {
            let width = text.content.len() as f64 * text.font_size * 0.6;
            let height = text.font_size * 1.2;
            let rect = Rect::new(
                text.position.x,
                text.position.y,
                text.position.x + width.max(20.0),
                text.position.y + height,
            );
            self.scene.fill(
                Fill::NonZero,
                transform,
                Color::from_rgba8(255, 100, 100, 100),
                None,
                &rect,
            );
        }
    }
}
