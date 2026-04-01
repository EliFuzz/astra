use crate::renderer::{GridStyle, RenderContext, Renderer, ShapeRenderer};
use crate::vello_impl::VelloRenderer;
use astra_canvas::camera::BASE_ZOOM;
use kurbo::{Affine, Point, Rect, RoundedRect, Stroke};
use peniko::{Color, Fill};

const LOD_INVISIBLE_MAX_AREA_PX2: f64 = 1.0;
const LOD_DOT_MAX_EDGE_PX: f64 = 3.0;
const LOD_SIMPLIFIED_MAX_AREA_PX2: f64 = 600.0;

impl Renderer for VelloRenderer {
    fn build_scene(&mut self, ctx: &RenderContext) {
        if ctx.viewport_size.width <= 0.0 || ctx.viewport_size.height <= 0.0 {
            return;
        }

        self.scene.reset();
        self.selection_color = ctx.selection_color;
        let zoom = ctx.canvas.camera.zoom;
        self.zoom = zoom;

        let camera_transform = ctx.canvas.camera.transform();
        let viewport = Rect::new(0.0, 0.0, ctx.viewport_size.width, ctx.viewport_size.height);

        let ratio = (zoom / BASE_ZOOM).max(f64::MIN_POSITIVE);
        let log_zoom = ratio.log10();
        let grid_exponent = (-log_zoom).floor();
        let base_grid_size = 20.0 * 10f64.powf(grid_exponent);

        match ctx.grid_style {
            GridStyle::None => {}
            GridStyle::Lines => self.render_grid_lines(viewport, camera_transform, base_grid_size),
            GridStyle::HorizontalLines => {
                self.render_horizontal_lines(viewport, camera_transform, base_grid_size)
            }
            GridStyle::CrossPlus => self.render_grid_crosses(viewport, camera_transform, base_grid_size),
            GridStyle::Dots => self.render_grid_dots(viewport, camera_transform, base_grid_size),
        }

        let render_margin = (50.0 / zoom).max(10.0);
        let cam = &ctx.canvas.camera;
        let world_viewport = Rect::new(
            -cam.offset.x / cam.zoom,
            -cam.offset.y / cam.zoom,
            (-cam.offset.x + ctx.viewport_size.width) / cam.zoom,
            (-cam.offset.y + ctx.viewport_size.height) / cam.zoom,
        );
        let inflated_viewport = world_viewport.inflate(render_margin, render_margin);

        let visible_shapes = ctx
            .canvas
            .document
            .visible_shapes_ordered(inflated_viewport);

        for shape in visible_shapes {
            if ctx.editing_shape_id == Some(shape.id()) {
                continue;
            }

            let shape_bounds = shape.bounds();
            let screen_w = shape_bounds.width() * zoom;
            let screen_h = shape_bounds.height() * zoom;
            let screen_area = screen_w * screen_h;

            if screen_area < LOD_INVISIBLE_MAX_AREA_PX2 {
                continue;
            }

            if screen_w < LOD_DOT_MAX_EDGE_PX && screen_h < LOD_DOT_MAX_EDGE_PX {
                render_dot_placeholder(
                    &mut self.scene,
                    shape_bounds.center(),
                    camera_transform,
                    zoom,
                );
                continue;
            }

            if screen_area < LOD_SIMPLIFIED_MAX_AREA_PX2 {
                render_simplified_placeholder(
                    &mut self.scene,
                    shape_bounds,
                    shape.style(),
                    camera_transform,
                );
                continue;
            }

            let is_selected = ctx.canvas.is_selected(shape.id());
            ShapeRenderer::render_shape(self, shape, camera_transform, is_selected);
        }

        if let Some(preview) = ctx.canvas.tool_manager.preview_shape() {
            ShapeRenderer::render_shape(self, &preview, camera_transform, false);
        }

        if let Some(rect) = ctx.selection_rect {
            self.render_selection_rect(rect, camera_transform);
        }

        if !ctx.smart_guides.is_empty() {
            self.render_smart_guides(&ctx.smart_guides, camera_transform);
        }

        if let Some(snap_point) = ctx.snap_point {
            self.render_snap_guides(snap_point, camera_transform, ctx.viewport_size);
        }

        if let Some(ref angle_info) = ctx.angle_snap_info {
            self.render_angle_snap_guides(angle_info, camera_transform, ctx.viewport_size);
        }

        if let Some(ref rotation_info) = ctx.rotation_info {
            self.render_rotation_guides(rotation_info, camera_transform);
        }

        if let Some((pos, radius)) = ctx.eraser_cursor {
            self.render_eraser_cursor(pos, radius, camera_transform);
        }

        if let Some((pos, ref trail)) = ctx.laser_pointer {
            self.render_laser_pointer(pos, trail, camera_transform);
        }

        if !ctx.arrow_snap_targets.is_empty() {
            self.render_arrow_snap_targets(&ctx.arrow_snap_targets, camera_transform);
        }
    }
}

fn render_dot_placeholder(
    scene: &mut vello::Scene,
    center: Point,
    transform: Affine,
    zoom: f64,
) {
    let r = (1.5 / zoom).max(0.5);
    let dot = kurbo::Circle::new(center, r);
    scene.fill(
        Fill::NonZero,
        transform,
        Color::from_rgba8(150, 150, 150, 180),
        None,
        &dot,
    );
}

fn render_simplified_placeholder(
    scene: &mut vello::Scene,
    bounds: Rect,
    style: &astra_core::ShapeStyle,
    transform: Affine,
) {
    let rect = RoundedRect::from_rect(bounds, 0.0);
    if let Some(fill) = style.fill_with_opacity() {
        scene.fill(Fill::NonZero, transform, fill, None, &rect);
    }
    let stroke = Stroke::new(style.stroke_width.min(1.5));
    scene.stroke(
        &stroke,
        transform,
        style.stroke_with_opacity(),
        None,
        &rect,
    );
}
