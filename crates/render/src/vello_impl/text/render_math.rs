use crate::vello_impl::VelloRenderer;
use crate::vello_impl::fonts;
use astra_canvas::shapes::ShapeTrait;
use kurbo::{Affine, Shape as KurboShape, Stroke};
use peniko::{Color, Fill};
use vello::Scene;

fn math_cache_key(latex: &str, font_size: f64, color: Color) -> (u64, u64, u32) {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    latex.hash(&mut hasher);
    let latex_hash = hasher.finish();
    let c = color.components;
    let r = (c[0].clamp(0.0, 1.0) * 255.0 + 0.5) as u64;
    let g = (c[1].clamp(0.0, 1.0) * 255.0 + 0.5) as u64;
    let b = (c[2].clamp(0.0, 1.0) * 255.0 + 0.5) as u64;
    let a = (c[3].clamp(0.0, 1.0) * 255.0 + 0.5) as u64;
    let color_packed = (r << 24) | (g << 16) | (b << 8) | a;
    (latex_hash, color_packed, (font_size as f32).to_bits())
}

fn math_placement_transform(
    math: &astra_canvas::shapes::Math,
    transform: Affine,
    w: f64,
    h: f64,
    depth: f64,
) -> Affine {
    let center_x = math.position.x + w / 2.0;
    let center_y = math.position.y - h / 2.0 - depth / 2.0;
    transform
        * Affine::translate((center_x, center_y))
        * Affine::rotate(math.rotation)
        * Affine::translate((-center_x, -center_y))
        * Affine::translate((math.position.x, math.position.y))
}

impl VelloRenderer {
    pub(crate) fn render_math(&mut self, math: &astra_canvas::shapes::Math, transform: Affine) {
        use crate::rex_backend::VelloBackend;
        use rex::font::backend::ttf_parser::TtfMathFont;
        use rex::layout::engine::LayoutBuilder;
        use rex::render::Renderer as RexRenderer;

        let color: Color = math.style.stroke_color.into();
        let cache_key = math_cache_key(&math.latex, math.font_size, color);

        self.generation = self.generation.wrapping_add(1);
        let touch = self.generation;
        if let Some(mut entry) = self.math_cache.remove(&cache_key) {
            entry.4 = touch;
            let (ref subscene, w, h, depth, _) = entry;
            math.set_cached_size(w, h, depth);
            let math_transform = math_placement_transform(math, transform, w, h, depth);
            self.scene.append(subscene, Some(math_transform));
            self.math_cache.insert(cache_key, entry);
            return;
        }

        let Ok(math_face) = ttf_parser::Face::parse(fonts::xits_math(), 0) else {
            self.render_math_error(math, transform, "Font parse error");
            return;
        };
        let Ok(math_font) = TtfMathFont::new(math_face) else {
            self.render_math_error(math, transform, "No MATH table");
            return;
        };
        let primary_face = ttf_parser::Face::parse(fonts::primary_math_face(), 0).ok();

        let Ok(parse_nodes) = rex::parser::parse(&math.latex) else {
            self.render_math_error(math, transform, "Parse error");
            return;
        };

        let layout_engine = LayoutBuilder::new(&math_font)
            .font_size(math.font_size)
            .build();
        let Ok(layout) = layout_engine.layout(&parse_nodes) else {
            self.render_math_error(math, transform, "Layout error");
            return;
        };

        let size = layout.size();
        math.set_cached_size(size.width, size.height, size.depth);

        let math_transform =
            math_placement_transform(math, transform, size.width, size.height, size.depth);

        let mut math_scene = Scene::new();
        let mut backend = VelloBackend::new(
            &mut math_scene,
            &math_font,
            primary_face.as_ref(),
            Affine::IDENTITY,
            color,
        );
        let renderer = RexRenderer::new();
        renderer.render(&layout, &mut backend);

        self.scene.append(&math_scene, Some(math_transform));
        while self.math_cache.len() > 200 {
            let remove_key = self
                .math_cache
                .iter()
                .min_by_key(|(_, v)| v.4)
                .map(|(k, _)| *k);
            match remove_key {
                Some(k) => {
                    self.math_cache.remove(&k);
                }
                None => break,
            }
        }
        self.math_cache.insert(
            cache_key,
            (math_scene, size.width, size.height, size.depth, touch),
        );
    }

    fn render_math_error(
        &mut self,
        math: &astra_canvas::shapes::Math,
        transform: Affine,
        _msg: &str,
    ) {
        let bounds = math.bounds();
        let rect_path = bounds.to_path(0.1);
        self.scene.fill(
            Fill::NonZero,
            transform,
            Color::from_rgba8(255, 200, 200, 100),
            None,
            &rect_path,
        );
        let stroke = Stroke::new(1.0);
        self.scene.stroke(
            &stroke,
            transform,
            Color::from_rgba8(255, 100, 100, 255),
            None,
            &rect_path,
        );
    }

    pub(crate) fn render_latex_at(
        &mut self,
        latex: &str,
        font_size: f64,
        x: f64,
        y: f64,
        color: Color,
        transform: Affine,
    ) -> (f64, f64) {
        use crate::rex_backend::VelloBackend;
        use rex::font::backend::ttf_parser::TtfMathFont;
        use rex::layout::engine::LayoutBuilder;
        use rex::render::Renderer as RexRenderer;

        let cache_key = math_cache_key(latex, font_size, color);

        self.generation = self.generation.wrapping_add(1);
        let touch = self.generation;
        if let Some(mut entry) = self.math_cache.remove(&cache_key) {
            entry.4 = touch;
            let (ref subscene, w, h, depth, _) = entry;
            let h_total = h + depth;
            let math_transform = transform * Affine::translate((x, y));
            self.scene.append(subscene, Some(math_transform));
            self.math_cache.insert(cache_key, entry);
            return (w, h_total);
        }

        let Ok(math_face) = ttf_parser::Face::parse(fonts::xits_math(), 0) else {
            return (font_size * latex.len() as f64 * 0.5, font_size);
        };
        let Ok(math_font) = TtfMathFont::new(math_face) else {
            return (font_size * latex.len() as f64 * 0.5, font_size);
        };
        let primary_face = ttf_parser::Face::parse(fonts::primary_math_face(), 0).ok();

        let Ok(parse_nodes) = rex::parser::parse(latex) else {
            return (font_size * latex.len() as f64 * 0.5, font_size);
        };

        let layout_engine = LayoutBuilder::new(&math_font).font_size(font_size).build();
        let Ok(layout) = layout_engine.layout(&parse_nodes) else {
            return (font_size * latex.len() as f64 * 0.5, font_size);
        };

        let size = layout.size();
        let math_transform = transform * Affine::translate((x, y));

        let mut math_scene = Scene::new();
        let mut backend = VelloBackend::new(
            &mut math_scene,
            &math_font,
            primary_face.as_ref(),
            Affine::IDENTITY,
            color,
        );
        let renderer = RexRenderer::new();
        renderer.render(&layout, &mut backend);

        self.scene.append(&math_scene, Some(math_transform));
        while self.math_cache.len() > 200 {
            let remove_key = self
                .math_cache
                .iter()
                .min_by_key(|(_, v)| v.4)
                .map(|(k, _)| *k);
            match remove_key {
                Some(k) => {
                    self.math_cache.remove(&k);
                }
                None => break,
            }
        }
        self.math_cache.insert(
            cache_key,
            (math_scene, size.width, size.height, size.depth, touch),
        );
        (size.width, size.height + size.depth)
    }
}
