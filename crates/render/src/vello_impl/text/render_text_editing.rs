use super::layout::convert_rect;
use crate::text_editor::TextEditState;
use crate::vello_impl::VelloRenderer;
use kurbo::{Affine, Point, Rect};
use parley::layout::PositionedLayoutItem;
use peniko::{Brush, Color, Fill};

impl VelloRenderer {
    pub fn render_text_editing(
        &mut self,
        text: &astra_canvas::shapes::Text,
        edit_state: &mut TextEditState,
        transform: Affine,
        anchor: Option<Point>,
    ) {
        use astra_canvas::shapes::FontWeight;

        let style = &text.style;
        let brush = Brush::Solid(style.stroke_with_opacity());

        let font_stack = text.font_family.system_font_stack();
        let parley_weight = match &text.font_weight {
            FontWeight::Light => parley::FontWeight::new(300.0),
            FontWeight::Regular => parley::FontWeight::NORMAL,
            FontWeight::Heavy => parley::FontWeight::BOLD,
        };

        edit_state.set_font_size(text.font_size as f32);
        edit_state.set_brush(brush.clone());

        {
            use parley::{FontStack, StyleProperty};
            let styles = edit_state.editor_mut().edit_styles();
            styles.insert(StyleProperty::FontStack(FontStack::Source(
                font_stack.into(),
            )));
            styles.insert(StyleProperty::FontWeight(parley_weight));
        }

        let editor_text: String = edit_state.editor().text().to_string();

        let mut builder =
            self.layout_cx
                .ranged_builder(&mut self.font_cx, &editor_text, 1.0, false);
        builder.push_default(parley::StyleProperty::FontSize(text.font_size as f32));
        builder.push_default(parley::StyleProperty::Brush(brush.clone()));
        builder.push_default(parley::StyleProperty::FontWeight(parley_weight));
        builder.push_default(parley::StyleProperty::FontStack(parley::FontStack::Source(
            font_stack.into(),
        )));

        let mut byte_offset = 0;
        for (char_idx, ch) in editor_text.chars().enumerate() {
            if let Some(Some(color)) = text.char_colors.get(char_idx) {
                let color: peniko::Color = (*color).into();
                let span_brush = Brush::Solid(color);
                let char_len = ch.len_utf8();
                builder.push(
                    parley::StyleProperty::Brush(span_brush),
                    byte_offset..byte_offset + char_len,
                );
            }
            byte_offset += ch.len_utf8();
        }

        let mut styled_layout = builder.build(&editor_text);
        styled_layout.break_all_lines(None);
        styled_layout.align(
            None,
            parley::Alignment::Start,
            parley::AlignmentOptions::default(),
        );

        let layout = edit_state
            .editor_mut()
            .layout(&mut self.font_cx, &mut self.layout_cx);
        let layout_width = layout.width() as f64;
        let layout_height = layout.height() as f64;

        text.set_cached_size(layout_width, layout_height);

        let rotation = text.rotation;
        let half_w = layout_width / 2.0;
        let half_h = layout_height / 2.0;

        let render_position = if let Some(anchor) = anchor {
            if rotation.abs() > 0.001 {
                let cos_r = rotation.cos();
                let sin_r = rotation.sin();
                let rot_x = -half_w * cos_r + half_h * sin_r;
                let rot_y = -half_w * sin_r - half_h * cos_r;
                Point::new(anchor.x - half_w - rot_x, anchor.y - half_h - rot_y)
            } else {
                anchor
            }
        } else {
            text.position
        };

        let text_transform = if rotation.abs() > 0.001 {
            let center = Point::new(render_position.x + half_w, render_position.y + half_h);
            let center_vec = kurbo::Vec2::new(center.x, center.y);
            transform
                * Affine::translate(center_vec)
                * Affine::rotate(rotation)
                * Affine::translate(-center_vec)
                * Affine::translate((render_position.x, render_position.y))
        } else {
            transform * Affine::translate((render_position.x, render_position.y))
        };

        for line in styled_layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let glyph_style = glyph_run.style();
                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let font_size = run.font_size();
                let synthesis = run.synthesis();
                let glyph_xform = synthesis
                    .skew()
                    .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));

                let glyphs: Vec<vello::Glyph> = glyph_run
                    .glyphs()
                    .map(|glyph| {
                        let gx = x + glyph.x;
                        let gy = y - glyph.y;
                        x += glyph.advance;
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
                        .brush(&glyph_style.brush)
                        .hint(true)
                        .transform(text_transform)
                        .glyph_transform(glyph_xform)
                        .font_size(font_size)
                        .normalized_coords(run.normalized_coords())
                        .draw(Fill::NonZero, glyphs.into_iter());
                }
            }
        }

        let selection_color = Color::from_rgba8(70, 130, 180, 128);

        edit_state.editor().selection_geometry_with(|rect, _| {
            self.scene.fill(
                Fill::NonZero,
                text_transform,
                selection_color,
                None,
                &convert_rect(&rect),
            );
        });

        if edit_state.is_cursor_visible() {
            if let Some(cursor) = edit_state.editor().cursor_geometry(1.5) {
                let cursor_color = Color::from_rgba8(0, 0, 0, 255);
                self.scene.fill(
                    Fill::NonZero,
                    text_transform,
                    cursor_color,
                    None,
                    &convert_rect(&cursor),
                );
            } else if edit_state.text().is_empty() {
                let cursor_height = text.font_size * 1.2;
                let cursor_rect = Rect::new(0.0, 0.0, 1.5, cursor_height);
                self.scene.fill(
                    Fill::NonZero,
                    text_transform,
                    Color::from_rgba8(0, 0, 0, 255),
                    None,
                    &cursor_rect,
                );
            }
        }
    }
}
