use super::mixed_segments::{parse_segments, TextSegment};
use super::render_text_mixed_plain_seg::MixedPlainSegParams;
use crate::vello_impl::VelloRenderer;
use kurbo::Affine;
use peniko::{Brush, Color};

impl VelloRenderer {
    pub(crate) fn render_text_mixed(
        &mut self,
        text: &astra_canvas::shapes::Text,
        transform: Affine,
    ) {
        use astra_canvas::shapes::FontWeight;

        let font_size = text.font_size;
        let line_height = font_size * 1.4;
        let text_color: Color = text.style.stroke_with_opacity();
        let brush = Brush::Solid(text_color);
        let font_stack = text.font_family.system_font_stack();
        let parley_weight = match text.font_weight {
            FontWeight::Light => parley::FontWeight::new(300.0),
            FontWeight::Regular => parley::FontWeight::NORMAL,
            FontWeight::Heavy => parley::FontWeight::BOLD,
        };

        let mut y_offset: f64 = 0.0;
        let mut total_width: f64 = 1.0;

        let total_char_count = text.content.chars().count();
        let mut content_char_pos: usize = 0;

        for source_line in text.content.split('\n') {
            let segments = parse_segments(source_line);
            let mut x_offset: f64 = 0.0;
            let mut line_h: f64 = line_height;

            for seg in &segments {
                match seg {
                    TextSegment::LaTeX(latex) => {
                        let render_x = text.position.x + x_offset;
                        let render_y = text.position.y + y_offset + font_size * 0.15;
                        let (w, h) = self.render_latex_at(
                            latex,
                            font_size,
                            render_x,
                            render_y,
                            text_color,
                            transform,
                        );
                        x_offset += w;
                        line_h = line_h.max(h + font_size * 0.15);
                        content_char_pos += 2 + latex.chars().count() + 2;
                    }
                    TextSegment::Plain(s) if !s.is_empty() => {
                        let seg_char_start = content_char_pos;
                        let seg_char_count = s.chars().count();
                        let (w, h) = self.render_mixed_plain_seg(MixedPlainSegParams {
                            s,
                            seg_char_start,
                            total_char_count,
                            text,
                            transform,
                            x_offset,
                            y_offset,
                            font_size,
                            brush: &brush,
                            parley_weight,
                            font_stack,
                        });
                        x_offset += w;
                        line_h = line_h.max(h);
                        content_char_pos += seg_char_count;
                    }
                    TextSegment::Plain(s) => {
                        content_char_pos += s.chars().count();
                    }
                }
            }

            content_char_pos += 1;

            total_width = total_width.max(x_offset);
            y_offset += line_h;
        }

        text.set_cached_size(total_width, y_offset.max(1.0));
    }
}
