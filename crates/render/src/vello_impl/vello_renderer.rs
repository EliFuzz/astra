use crate::renderer::ShapeRenderer;
use super::cache::CachedTextLayout;
use kurbo::{Affine, BezPath, Rect};
use parley::{FontContext, LayoutContext};
use peniko::{Brush, Color};
use vello::Scene;

type ShapeCacheKey = (String, u32, u32, u64, i32, u64);
type MathCacheKey = (u64, u64, u32);
type MathCacheEntry = (Scene, f64, f64, f64, u64);

pub struct VelloRenderer {
    pub(crate) scene: Scene,
    pub(crate) selection_color: Color,
    pub(crate) font_cx: FontContext,
    pub(crate) layout_cx: LayoutContext<Brush>,
    pub(crate) zoom: f64,
    pub(crate) image_cache: std::collections::HashMap<String, peniko::ImageData>,
    pub(crate) shape_cache: std::collections::HashMap<ShapeCacheKey, (BezPath, u64)>,
    pub(crate) text_cache: std::collections::HashMap<(String, u64), CachedTextLayout>,
    pub(crate) math_cache: std::collections::HashMap<MathCacheKey, MathCacheEntry>,
    pub(crate) generation: u64,
    pub(crate) text_generation: u64,
}

impl Default for VelloRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl VelloRenderer {
    pub fn new() -> Self {
        let mut font_cx = FontContext::new();
        super::fonts::load_and_register(&mut font_cx);

        Self {
            scene: Scene::new(),
            selection_color: Color::from_rgba8(59, 130, 246, 255),
            font_cx,
            layout_cx: LayoutContext::new(),
            zoom: 1.0,
            image_cache: std::collections::HashMap::new(),
            shape_cache: std::collections::HashMap::new(),
            text_cache: std::collections::HashMap::new(),
            math_cache: std::collections::HashMap::new(),
            generation: 0,
            text_generation: 0,
        }
    }

    pub fn new_for_export(scale: f64) -> Self {
        let mut r = Self::new();
        r.zoom = scale;
        r
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn take_scene(&mut self) -> Scene {
        std::mem::take(&mut self.scene)
    }

    pub fn contexts_mut(&mut self) -> (&mut FontContext, &mut LayoutContext<Brush>) {
        (&mut self.font_cx, &mut self.layout_cx)
    }

    fn compute_selection_bounds(
        document: &astra_canvas::canvas::CanvasDocument,
        selection: Option<&[astra_canvas::shapes::ShapeId]>,
    ) -> Option<Rect> {
        match selection {
            None => document.bounds(),
            Some(&[]) => None,
            Some(ids) => {
                let mut min_x = f64::MAX;
                let mut min_y = f64::MAX;
                let mut max_x = f64::MIN;
                let mut max_y = f64::MIN;
                let mut any = false;
                for &shape_id in ids {
                    if let Some(shape) = document.get_shape(shape_id) {
                        let b = shape.bounds();
                        min_x = min_x.min(b.x0);
                        min_y = min_y.min(b.y0);
                        max_x = max_x.max(b.x1);
                        max_y = max_y.max(b.y1);
                        any = true;
                    }
                }
                any.then_some(Rect::new(min_x, min_y, max_x, max_y))
            }
        }
    }

    pub fn build_export_scene(
        &mut self,
        document: &astra_canvas::canvas::CanvasDocument,
        scale: f64,
    ) -> (Scene, Option<Rect>) {
        self.scene.reset();
        self.zoom = scale;

        let Some(bounds) = Self::compute_selection_bounds(document, None) else {
            return (std::mem::take(&mut self.scene), None);
        };

        let padding = 20.0;
        let padded_bounds = bounds.inflate(padding, padding);
        let transform =
            Affine::scale(scale) * Affine::translate((-padded_bounds.x0, -padded_bounds.y0));
        let scaled_width = padded_bounds.width() * scale;
        let scaled_height = padded_bounds.height() * scale;

        let bg_rect = Rect::new(0.0, 0.0, scaled_width, scaled_height);
        self.scene.fill(
            peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::WHITE,
            None,
            &bg_rect,
        );

        for shape in document.shapes_ordered() {
            ShapeRenderer::render_shape(self, shape, transform, false);
        }

        let scaled_bounds = Rect::new(0.0, 0.0, scaled_width, scaled_height);
        (std::mem::take(&mut self.scene), Some(scaled_bounds))
    }

    pub fn build_export_scene_selection(
        &mut self,
        document: &astra_canvas::canvas::CanvasDocument,
        selection: &[astra_canvas::shapes::ShapeId],
        scale: f64,
    ) -> (Scene, Option<Rect>) {
        self.scene.reset();
        self.zoom = scale;

        let Some(bounds) = Self::compute_selection_bounds(document, Some(selection)) else {
            return (std::mem::take(&mut self.scene), None);
        };

        let padding = 20.0;
        let padded_bounds = bounds.inflate(padding, padding);
        let transform =
            Affine::scale(scale) * Affine::translate((-padded_bounds.x0, -padded_bounds.y0));
        let scaled_width = padded_bounds.width() * scale;
        let scaled_height = padded_bounds.height() * scale;

        let bg_rect = Rect::new(0.0, 0.0, scaled_width, scaled_height);
        self.scene.fill(
            peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::WHITE,
            None,
            &bg_rect,
        );

        let mut shapes_to_render = Vec::new();
        for &shape_id in selection {
            if let Some(shape) = document.get_shape(shape_id) {
                shapes_to_render.push(shape);
            }
        }

        for shape in shapes_to_render {
            ShapeRenderer::render_shape(self, shape, transform, false);
        }

        let scaled_bounds = Rect::new(0.0, 0.0, scaled_width, scaled_height);
        (std::mem::take(&mut self.scene), Some(scaled_bounds))
    }
}
