use super::super::VelloRenderer;
use kurbo::{Affine, BezPath, Point, Stroke};
use peniko::{Color, Fill};

impl VelloRenderer {
    pub(super) fn render_eraser_cursor(&mut self, pos: Point, radius: f64, transform: Affine) {
        let circle = kurbo::Circle::new(pos, radius);

        let fill_color = Color::from_rgba8(255, 100, 100, 50);
        self.scene
            .fill(Fill::NonZero, transform, fill_color, None, &circle);

        let stroke_width = 2.0 / self.zoom;
        let stroke_color = Color::from_rgba8(200, 50, 50, 200);
        self.scene.stroke(
            &Stroke::new(stroke_width),
            transform,
            stroke_color,
            None,
            &circle,
        );
    }

    pub(super) fn render_laser_pointer(
        &mut self,
        pos: Point,
        trail: &[(Point, f64)],
        transform: Affine,
    ) {
        let base_width = 4.0 / self.zoom;
        for pair in trail.windows(2) {
            let (p0, a0) = pair[0];
            let (p1, a1) = pair[1];
            let avg_alpha = ((a0 + a1) * 0.5 * 255.0) as u8;
            if avg_alpha == 0 {
                continue;
            }
            let color = Color::from_rgba8(255, 0, 0, avg_alpha);
            let width = base_width * (a0 + a1) * 0.5;
            let mut seg = BezPath::new();
            seg.move_to(p0);
            seg.line_to(p1);
            self.scene.stroke(
                &Stroke::new(width.max(0.5 / self.zoom)),
                transform,
                color,
                None,
                &seg,
            );
        }

        let glow_radius = 10.0 / self.zoom;
        let glow = kurbo::Circle::new(pos, glow_radius);
        self.scene.fill(
            Fill::NonZero,
            transform,
            Color::from_rgba8(255, 0, 0, 80),
            None,
            &glow,
        );

        let main_radius = 5.0 / self.zoom;
        let main = kurbo::Circle::new(pos, main_radius);
        self.scene.fill(
            Fill::NonZero,
            transform,
            Color::from_rgba8(255, 50, 50, 255),
            None,
            &main,
        );

        let center_radius = 1.5 / self.zoom;
        let center = kurbo::Circle::new(pos, center_radius);
        self.scene
            .fill(Fill::NonZero, transform, Color::WHITE, None, &center);
    }

    pub(super) fn render_snap_guides(
        &mut self,
        snap_point: Point,
        transform: Affine,
        viewport_size: kurbo::Size,
    ) {
        let guide_color = Color::from_rgba8(236, 72, 153, 180);
        let stroke_width = 1.0 / self.zoom;
        let stroke = Stroke::new(stroke_width);

        let inv_transform = transform.inverse();
        let world_top_left = inv_transform * Point::new(0.0, 0.0);
        let world_bottom_right =
            inv_transform * Point::new(viewport_size.width, viewport_size.height);

        let mut h_path = BezPath::new();
        h_path.move_to(Point::new(world_top_left.x, snap_point.y));
        h_path.line_to(Point::new(world_bottom_right.x, snap_point.y));
        self.scene
            .stroke(&stroke, transform, guide_color, None, &h_path);

        let mut v_path = BezPath::new();
        v_path.move_to(Point::new(snap_point.x, world_top_left.y));
        v_path.line_to(Point::new(snap_point.x, world_bottom_right.y));
        self.scene
            .stroke(&stroke, transform, guide_color, None, &v_path);

        let circle_radius = 4.0 / self.zoom;
        let circle = kurbo::Circle::new(snap_point, circle_radius);
        self.scene.stroke(
            &Stroke::new(stroke_width * 2.0),
            transform,
            guide_color,
            None,
            &circle,
        );
    }

    pub(super) fn render_arrow_snap_targets(&mut self, targets: &[Point], transform: Affine) {
        let radius = 6.0 / self.zoom;
        let stroke_width = 2.0 / self.zoom;
        let fill_color = Color::from_rgba8(34, 197, 94, 80);
        let stroke_color = Color::from_rgba8(34, 197, 94, 220);
        for &pt in targets {
            let circle = kurbo::Circle::new(pt, radius);
            self.scene
                .fill(Fill::NonZero, transform, fill_color, None, &circle);
            self.scene.stroke(
                &Stroke::new(stroke_width),
                transform,
                stroke_color,
                None,
                &circle,
            );
        }
    }
}
