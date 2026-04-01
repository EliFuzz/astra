use kurbo::{Affine, Point, Size, Vec2};
use serde::{Deserialize, Serialize};

pub const BASE_ZOOM: f64 = 1.68;

pub(crate) const ZOOM_SOFT_MIN: f64 = 0.00168;
pub(crate) const ZOOM_SOFT_MAX: f64 = 50.4;
const WORLD_REBASE_THRESHOLD: f64 = 1e6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub offset: Vec2,
    pub zoom: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: BASE_ZOOM,
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transform(&self) -> Affine {
        Affine::translate(self.offset) * Affine::scale(self.zoom)
    }

    pub fn inverse_transform(&self) -> Affine {
        Affine::scale(1.0 / self.zoom) * Affine::translate(-self.offset)
    }

    pub fn screen_to_world(&self, screen_point: Point) -> Point {
        self.inverse_transform() * screen_point
    }

    pub fn world_to_screen(&self, world_point: Point) -> Point {
        self.transform() * world_point
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.offset += delta;
    }

    pub fn zoom_level(&self) -> f64 {
        (self.zoom / BASE_ZOOM).log2()
    }

    pub fn set_zoom_level(&mut self, level: f64, screen_anchor: Point) {
        if !level.is_finite() {
            return;
        }
        let new_zoom = (BASE_ZOOM * level.exp2()).clamp(ZOOM_SOFT_MIN, ZOOM_SOFT_MAX);
        self.apply_zoom_keeping_anchor(new_zoom, screen_anchor);
    }

    pub fn zoom_at(&mut self, screen_point: Point, factor: f64) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let log_zoom = self.zoom.ln() + factor.ln();
        let new_zoom = log_zoom.exp().clamp(ZOOM_SOFT_MIN, ZOOM_SOFT_MAX);
        self.apply_zoom_keeping_anchor(new_zoom, screen_point);
    }

    pub fn reset(&mut self) {
        self.offset = Vec2::ZERO;
        self.zoom = BASE_ZOOM;
    }

    pub fn viewport_center_world(&self, viewport: Size) -> Point {
        self.screen_to_world(Point::new(viewport.width / 2.0, viewport.height / 2.0))
    }

    pub fn fit_to_bounds(&mut self, bounds: kurbo::Rect, viewport: kurbo::Size, padding: f64) {
        if bounds.is_zero_area() {
            self.reset();
            return;
        }
        let padded_viewport = kurbo::Size::new(
            (viewport.width - padding * 2.0).max(1.0),
            (viewport.height - padding * 2.0).max(1.0),
        );
        let scale_x = padded_viewport.width / bounds.width();
        let scale_y = padded_viewport.height / bounds.height();
        self.zoom = scale_x.min(scale_y).clamp(ZOOM_SOFT_MIN, ZOOM_SOFT_MAX);
        let bounds_center = bounds.center();
        let viewport_center = Point::new(viewport.width / 2.0, viewport.height / 2.0);
        self.offset = Vec2::new(
            viewport_center.x - bounds_center.x * self.zoom,
            viewport_center.y - bounds_center.y * self.zoom,
        );
    }

    pub fn origin_rebased_transform(&self) -> Affine {
        let anchor = self.screen_to_world(Point::ZERO);
        let ax = anchor.x.abs();
        let ay = anchor.y.abs();
        if ax <= WORLD_REBASE_THRESHOLD && ay <= WORLD_REBASE_THRESHOLD {
            return self.transform();
        }
        let rebased_offset = Vec2::new(
            self.offset.x + self.zoom * anchor.x,
            self.offset.y + self.zoom * anchor.y,
        );
        Affine::translate(rebased_offset)
            * Affine::scale(self.zoom)
            * Affine::translate(Vec2::new(-anchor.x, -anchor.y))
    }

    fn apply_zoom_keeping_anchor(&mut self, new_zoom: f64, screen_anchor: Point) {
        if !new_zoom.is_finite() || (new_zoom - self.zoom).abs() < f64::EPSILON {
            return;
        }
        let world_point = self.screen_to_world(screen_anchor);
        self.zoom = new_zoom;
        let new_screen = self.world_to_screen(world_point);
        self.offset += Vec2::new(
            screen_anchor.x - new_screen.x,
            screen_anchor.y - new_screen.y,
        );
    }
}
