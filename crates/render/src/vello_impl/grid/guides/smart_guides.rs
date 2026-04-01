use crate::vello_impl::VelloRenderer;
use kurbo::{Affine, BezPath, Point, Rect, Stroke};
use peniko::{Brush, Color, Fill};

impl VelloRenderer {
    fn render_badge(&mut self, text: &str, center: Point, bg_color: Color, transform: Affine) {
        use parley::{PositionedLayoutItem, StyleProperty};

        let font_size = 11.0_f32;
        let padding = 3.0 / self.zoom;

        let mut builder = self
            .layout_cx
            .ranged_builder(&mut self.font_cx, text, 1.0, false);
        builder.push_default(StyleProperty::FontSize(font_size));
        builder.push_default(StyleProperty::Brush(Brush::Solid(Color::WHITE)));
        builder.push_default(StyleProperty::FontStack(parley::FontStack::Source(
            astra_canvas::shapes::FontFamily::Sans
                .system_font_stack()
                .into(),
        )));
        let mut layout = builder.build(text);
        layout.break_all_lines(None);

        let text_width = layout.width() as f64 / self.zoom;
        let text_height = layout.height() as f64 / self.zoom;

        let rect = Rect::new(
            center.x - text_width / 2.0 - padding,
            center.y - text_height / 2.0 - padding,
            center.x + text_width / 2.0 + padding,
            center.y + text_height / 2.0 + padding,
        );
        self.scene
            .fill(Fill::NonZero, transform, bg_color, None, &rect);

        let text_x = center.x - text_width / 2.0;
        let text_y = center.y - text_height / 2.0;
        let text_transform =
            transform * Affine::translate((text_x, text_y)) * Affine::scale(1.0 / self.zoom);

        for line in layout.lines() {
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                    continue;
                };
                let mut x = glyph_run.offset();
                let y = glyph_run.baseline();
                let run = glyph_run.run();
                let font = run.font();
                let font_size = run.font_size();

                let glyphs: Vec<vello::Glyph> = glyph_run
                    .glyphs()
                    .map(|g| {
                        let gx = x + g.x;
                        let gy = y - g.y;
                        x += g.advance;
                        vello::Glyph {
                            id: g.id,
                            x: gx,
                            y: gy,
                        }
                    })
                    .collect();

                if !glyphs.is_empty() {
                    self.scene
                        .draw_glyphs(font)
                        .brush(&Brush::Solid(Color::WHITE))
                        .hint(true)
                        .transform(text_transform)
                        .font_size(font_size)
                        .normalized_coords(run.normalized_coords())
                        .draw(Fill::NonZero, glyphs.into_iter());
                }
            }
        }
    }

    pub(in crate::vello_impl::grid) fn render_smart_guides(
        &mut self,
        guides: &[astra_canvas::snap::SmartGuide],
        transform: Affine,
    ) {
        use astra_canvas::snap::SmartGuideKind;

        let guide_color = Color::from_rgba8(236, 72, 153, 255);
        let stroke_width = 1.0 / self.zoom;
        let stroke = Stroke::new(stroke_width);
        let cap_size = 4.0 / self.zoom;
        let x_size = 3.0 / self.zoom;
        let mut badges: Vec<(String, Point)> = Vec::new();

        for guide in guides {
            let mut path = BezPath::new();
            match guide.kind {
                SmartGuideKind::Vertical => {
                    path.move_to(Point::new(guide.position, guide.start));
                    path.line_to(Point::new(guide.position, guide.end));
                    for &y in &guide.snap_points {
                        path.move_to(Point::new(guide.position - x_size, y - x_size));
                        path.line_to(Point::new(guide.position + x_size, y + x_size));
                        path.move_to(Point::new(guide.position - x_size, y + x_size));
                        path.line_to(Point::new(guide.position + x_size, y - x_size));
                    }
                }
                SmartGuideKind::Horizontal => {
                    path.move_to(Point::new(guide.start, guide.position));
                    path.line_to(Point::new(guide.end, guide.position));
                    for &x in &guide.snap_points {
                        path.move_to(Point::new(x - x_size, guide.position - x_size));
                        path.line_to(Point::new(x + x_size, guide.position + x_size));
                        path.move_to(Point::new(x - x_size, guide.position + x_size));
                        path.line_to(Point::new(x + x_size, guide.position - x_size));
                    }
                }
                SmartGuideKind::EqualSpacingH => {
                    let y = guide.position;
                    path.move_to(Point::new(guide.start, y));
                    path.line_to(Point::new(guide.end, y));
                    path.move_to(Point::new(guide.start, y - cap_size));
                    path.line_to(Point::new(guide.start, y + cap_size));
                    path.move_to(Point::new(guide.end, y - cap_size));
                    path.line_to(Point::new(guide.end, y + cap_size));
                    let dist = (guide.end - guide.start).abs();
                    let center = Point::new((guide.start + guide.end) / 2.0, y);
                    badges.push((format!("{:.0}", dist), center));
                }
                SmartGuideKind::EqualSpacingV => {
                    let x = guide.position;
                    path.move_to(Point::new(x, guide.start));
                    path.line_to(Point::new(x, guide.end));
                    path.move_to(Point::new(x - cap_size, guide.start));
                    path.line_to(Point::new(x + cap_size, guide.start));
                    path.move_to(Point::new(x - cap_size, guide.end));
                    path.line_to(Point::new(x + cap_size, guide.end));
                    let dist = (guide.end - guide.start).abs();
                    let center = Point::new(x, (guide.start + guide.end) / 2.0);
                    badges.push((format!("{:.0}", dist), center));
                }
            }
            self.scene
                .stroke(&stroke, transform, guide_color, None, &path);
        }

        for (text, center) in badges {
            self.render_badge(&text, center, guide_color, transform);
        }
    }
}
