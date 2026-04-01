use crate::vello_impl::VelloRenderer;
use kurbo::Affine;
use parley::layout::PositionedLayoutItem;
use peniko::{Brush, Fill};

pub(crate) struct MixedPlainSegParams<'a> {
    pub s: &'a str,
    pub seg_char_start: usize,
    pub total_char_count: usize,
    pub text: &'a astra_canvas::shapes::Text,
    pub transform: Affine,
    pub x_offset: f64,
    pub y_offset: f64,
    pub font_size: f64,
    pub brush: &'a Brush,
    pub parley_weight: parley::FontWeight,
    pub font_stack: &'static str,
}

impl VelloRenderer {
    pub(crate) fn render_mixed_plain_seg(
        &mut self,
        params: MixedPlainSegParams<'_>,
    ) -> (f64, f64) {
        use parley::StyleProperty;

        let MixedPlainSegParams {
            s,
            seg_char_start,
            total_char_count,
            text,
            transform,
            x_offset,
            y_offset,
            font_size,
            brush,
            parley_weight,
            font_stack,
        } = params;

        let mut builder = self.layout_cx.ranged_builder(&mut self.font_cx, s, 1.0, false);
        builder.push_default(StyleProperty::FontSize(font_size as f32));
        builder.push_default(StyleProperty::Brush(brush.clone()));
        builder.push_default(StyleProperty::FontWeight(parley_weight));
        builder.push_default(StyleProperty::FontStack(parley::FontStack::Source(
            font_stack.into(),
        )));

        let mut byte_offset_in_seg = 0;
        for (i, ch) in s.chars().enumerate() {
            let char_idx = seg_char_start + i;
            if char_idx < total_char_count {
                if let Some(Some(color)) = text.char_colors.get(char_idx) {
                    let c: peniko::Color = (*color).into();
                    let span_brush = Brush::Solid(c);
                    let char_len = ch.len_utf8();
                    builder.push(
                        StyleProperty::Brush(span_brush),
                        byte_offset_in_seg..byte_offset_in_seg + char_len,
                    );
                }
            }
            byte_offset_in_seg += ch.len_utf8();
        }

        let mut layout = builder.build(s);
        layout.break_all_lines(None);
        layout.align(
            None,
            parley::Alignment::Start,
            parley::AlignmentOptions::default(),
        );

        let seg_h = layout.height() as f64;

        let seg_transform = transform
            * Affine::translate((text.position.x + x_offset, text.position.y + y_offset));

        let mut advance_width: f64 = 0.0;

        for line in layout.lines() {
            let mut line_advance: f64 = 0.0;
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let run_offset = glyph_run.offset() as f64;
                let mut gx = glyph_run.offset();
                let gy = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let run_font_size = run.font_size();
                let synthesis = run.synthesis();
                let skew_angle = synthesis.skew().map(|a| a.to_radians().tan() as f64);
                let glyph_xform = skew_angle.map(|a| Affine::skew(a, 0.0));

                let mut run_advance: f64 = 0.0;
                let glyphs: Vec<vello::Glyph> = glyph_run
                    .glyphs()
                    .map(|g| {
                        let px = gx + g.x;
                        let py = gy - g.y;
                        gx += g.advance;
                        run_advance += g.advance as f64;
                        vello::Glyph {
                            id: g.id,
                            x: px,
                            y: py,
                        }
                    })
                    .collect();

                line_advance = line_advance.max(run_offset + run_advance);

                if !glyphs.is_empty() {
                    self.scene
                        .draw_glyphs(font)
                        .brush(&glyph_run.style().brush)
                        .hint(true)
                        .transform(seg_transform)
                        .glyph_transform(glyph_xform)
                        .font_size(run_font_size)
                        .normalized_coords(run.normalized_coords())
                        .draw(Fill::NonZero, glyphs.iter().cloned());
                }
            }
            advance_width = advance_width.max(line_advance);
        }

        let seg_w = if advance_width > 0.0 {
            advance_width
        } else {
            layout.width() as f64
        };

        (seg_w, seg_h)
    }
}
