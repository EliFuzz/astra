use super::super::super::VelloRenderer;
use crate::renderer::{AngleSnapInfo, RotationInfo};
use kurbo::{Affine, BezPath, Point, Stroke};
use peniko::Color;

impl VelloRenderer {
    pub(crate) fn render_angle_snap_guides(
        &mut self,
        info: &AngleSnapInfo,
        transform: Affine,
        viewport_size: kurbo::Size,
    ) {
        use std::f64::consts::PI;

        let ray_color = Color::from_rgba8(100, 100, 100, 60);
        let active_ray_color = Color::from_rgba8(236, 72, 153, 200);
        let arc_color = Color::from_rgba8(236, 72, 153, 220);

        let thin_stroke_width = 0.5 / self.zoom;
        let thick_stroke_width = 1.5 / self.zoom;

        let start = info.start_point;

        let inv_transform = transform.inverse();
        let world_top_left = inv_transform * Point::new(0.0, 0.0);
        let world_bottom_right =
            inv_transform * Point::new(viewport_size.width, viewport_size.height);
        let viewport_diagonal = ((world_bottom_right.x - world_top_left.x).powi(2)
            + (world_bottom_right.y - world_top_left.y).powi(2))
        .sqrt();
        let ray_length = viewport_diagonal;

        let mut path = BezPath::new();
        for i in 0..24 {
            let angle_deg = i as f64 * 15.0;
            let angle_rad = angle_deg * PI / 180.0;
            let end_x = start.x + ray_length * angle_rad.cos();
            let end_y = start.y + ray_length * angle_rad.sin();
            path.move_to(start);
            path.line_to(Point::new(end_x, end_y));
        }
        self.scene.stroke(
            &Stroke::new(thin_stroke_width),
            transform,
            ray_color,
            None,
            &path,
        );

        if info.is_snapped {
            let angle_rad = info.angle_degrees * PI / 180.0;
            let mut active_path = BezPath::new();
            active_path.move_to(start);
            active_path.line_to(Point::new(
                start.x + ray_length * angle_rad.cos(),
                start.y + ray_length * angle_rad.sin(),
            ));
            self.scene.stroke(
                &Stroke::new(thick_stroke_width),
                transform,
                active_ray_color,
                None,
                &active_path,
            );

            let arc_radius = 30.0 / self.zoom;
            let segments = (info.angle_degrees.abs() / 5.0).ceil() as usize;
            let segments = segments.clamp(2, 72);

            if segments > 1 {
                let mut arc_path = BezPath::new();
                let start_angle = 0.0_f64;
                let end_angle = info.angle_degrees * PI / 180.0;

                let first_x = start.x + arc_radius * start_angle.cos();
                let first_y = start.y + arc_radius * start_angle.sin();
                arc_path.move_to(Point::new(first_x, first_y));

                for i in 1..=segments {
                    let t = i as f64 / segments as f64;
                    let angle = start_angle + t * (end_angle - start_angle);
                    let x = start.x + arc_radius * angle.cos();
                    let y = start.y + arc_radius * angle.sin();
                    arc_path.line_to(Point::new(x, y));
                }

                self.scene.stroke(
                    &Stroke::new(thick_stroke_width),
                    transform,
                    arc_color,
                    None,
                    &arc_path,
                );
            }
        }
    }

    pub(crate) fn render_rotation_guides(&mut self, info: &RotationInfo, transform: Affine) {
        use std::f64::consts::PI;

        let guide_color = Color::from_rgba8(236, 72, 153, 200);
        let snap_color = Color::from_rgba8(34, 197, 94, 220);

        let color = if info.snapped {
            snap_color
        } else {
            guide_color
        };

        let stroke_width = 1.5 / self.zoom;
        let center = info.center;

        let cross_size = 10.0 / self.zoom;
        let mut cross_path = BezPath::new();
        cross_path.move_to(Point::new(center.x - cross_size, center.y));
        cross_path.line_to(Point::new(center.x + cross_size, center.y));
        cross_path.move_to(Point::new(center.x, center.y - cross_size));
        cross_path.line_to(Point::new(center.x, center.y + cross_size));
        self.scene.stroke(
            &Stroke::new(stroke_width),
            transform,
            color,
            None,
            &cross_path,
        );

        let indicator_length = 50.0 / self.zoom;
        let angle = info.angle - PI / 2.0;
        let end_x = center.x + indicator_length * angle.cos();
        let end_y = center.y + indicator_length * angle.sin();

        let mut indicator_path = BezPath::new();
        indicator_path.move_to(center);
        indicator_path.line_to(Point::new(end_x, end_y));
        self.scene.stroke(
            &Stroke::new(stroke_width * 1.5),
            transform,
            color,
            None,
            &indicator_path,
        );

        let ref_end_y = center.y - indicator_length;
        let mut ref_path = BezPath::new();
        ref_path.move_to(center);
        ref_path.line_to(Point::new(center.x, ref_end_y));

        let dash_pattern = [4.0 / self.zoom, 4.0 / self.zoom];
        let dashed_stroke = Stroke::new(stroke_width).with_dashes(0.0, dash_pattern);
        self.scene.stroke(
            &dashed_stroke,
            transform,
            Color::from_rgba8(150, 150, 150, 150),
            None,
            &ref_path,
        );

        if info.angle.abs() > 0.01 {
            let arc_radius = 25.0 / self.zoom;
            let segments = ((info.angle.abs() * 180.0 / PI) / 5.0).ceil() as usize;
            let segments = segments.clamp(2, 72);

            let mut arc_path = BezPath::new();
            let start_angle = -PI / 2.0;
            let end_angle = info.angle - PI / 2.0;

            let first_x = center.x + arc_radius * start_angle.cos();
            let first_y = center.y + arc_radius * start_angle.sin();
            arc_path.move_to(Point::new(first_x, first_y));

            for i in 1..=segments {
                let t = i as f64 / segments as f64;
                let a = start_angle + t * (end_angle - start_angle);
                let x = center.x + arc_radius * a.cos();
                let y = center.y + arc_radius * a.sin();
                arc_path.line_to(Point::new(x, y));
            }

            self.scene.stroke(
                &Stroke::new(stroke_width),
                transform,
                color,
                None,
                &arc_path,
            );
        }
    }
}
