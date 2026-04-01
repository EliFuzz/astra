use super::super::VelloRenderer;
use astra_canvas::selection::{Handle, HandleKind, get_handles};
use astra_canvas::shapes::Shape;
use kurbo::{Affine, BezPath, Point, Rect, Shape as KurboShape, Stroke};
use peniko::{Color, Fill};

impl VelloRenderer {
    pub(crate) fn render_shape_handles(&mut self, shape: &Shape, transform: Affine) {
        let handles = get_handles(shape);
        let handle_size = 16.0 / self.zoom;
        let stroke_width = 1.0 / self.zoom;
        let dash_len = 4.0 / self.zoom;

        match shape {
            Shape::Line(_) | Shape::Arrow(_) => {}
            _ => {
                let bounds = shape.bounds();
                let rotation = shape.rotation();
                let stroke = Stroke::new(stroke_width).with_dashes(0.0, [dash_len, dash_len]);

                let mut path = BezPath::new();
                path.move_to(Point::new(bounds.x0, bounds.y0));
                path.line_to(Point::new(bounds.x1, bounds.y0));
                path.line_to(Point::new(bounds.x1, bounds.y1));
                path.line_to(Point::new(bounds.x0, bounds.y1));
                path.close_path();

                let box_transform = if rotation.abs() > 0.001 {
                    let center = bounds.center();
                    transform
                        * Affine::translate((center.x, center.y))
                        * Affine::rotate(rotation)
                        * Affine::translate((-center.x, -center.y))
                } else {
                    transform
                };

                self.scene
                    .stroke(&stroke, box_transform, self.selection_color, None, &path);
            }
        }

        for handle in handles {
            self.render_handle(&handle, transform, handle_size);
        }
    }

    fn render_handle(&mut self, handle: &Handle, transform: Affine, size: f64) {
        let pos = handle.position;
        let stroke_width_thick = 2.0 / self.zoom;
        let stroke_width_thin = 1.5 / self.zoom;

        match handle.kind {
            HandleKind::Endpoint(_) | HandleKind::IntermediatePoint(_) => {
                let radius = size / 2.0;
                let ellipse = kurbo::Ellipse::new(pos, (radius, radius), 0.0);
                let path = ellipse.to_path(0.1);

                self.scene
                    .fill(Fill::NonZero, transform, Color::WHITE, None, &path);
                self.scene.stroke(
                    &Stroke::new(stroke_width_thick),
                    transform,
                    self.selection_color,
                    None,
                    &path,
                );
            }
            HandleKind::SegmentMidpoint(_) => {
                let radius = size / 3.0;
                let ellipse = kurbo::Ellipse::new(pos, (radius, radius), 0.0);
                let path = ellipse.to_path(0.1);

                self.scene.fill(
                    Fill::NonZero,
                    transform,
                    Color::from_rgba8(200, 220, 255, 200),
                    None,
                    &path,
                );
                self.scene.stroke(
                    &Stroke::new(stroke_width_thin),
                    transform,
                    self.selection_color,
                    None,
                    &path,
                );
            }
            HandleKind::Corner(_) | HandleKind::Edge(_) => {
                let half = size / 2.0;
                let rect = Rect::new(pos.x - half, pos.y - half, pos.x + half, pos.y + half);
                let path = rect.to_path(0.1);

                self.scene
                    .fill(Fill::NonZero, transform, Color::WHITE, None, &path);
                self.scene.stroke(
                    &Stroke::new(stroke_width_thin),
                    transform,
                    self.selection_color,
                    None,
                    &path,
                );
            }
            HandleKind::Rotate => {
                let radius = size / 2.0;
                let ellipse = kurbo::Ellipse::new(pos, (radius, radius), 0.0);
                let path = ellipse.to_path(0.1);

                self.scene
                    .fill(Fill::NonZero, transform, Color::WHITE, None, &path);
                self.scene.stroke(
                    &Stroke::new(stroke_width_thick),
                    transform,
                    self.selection_color,
                    None,
                    &path,
                );

                let arc_radius = radius * 0.5;
                let mut arc_path = BezPath::new();
                let start_angle = -std::f64::consts::FRAC_PI_4;
                let end_angle = start_angle + std::f64::consts::PI * 1.5;
                let steps = 12;
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let angle = start_angle + t * (end_angle - start_angle);
                    let x = pos.x + arc_radius * angle.cos();
                    let y = pos.y + arc_radius * angle.sin();
                    if i == 0 {
                        arc_path.move_to(Point::new(x, y));
                    } else {
                        arc_path.line_to(Point::new(x, y));
                    }
                }
                self.scene.stroke(
                    &Stroke::new(stroke_width_thin),
                    transform,
                    self.selection_color,
                    None,
                    &arc_path,
                );
            }
        }
    }

    pub(super) fn render_selection_rect(&mut self, rect: Rect, transform: Affine) {
        let fill_color = Color::from_rgba8(59, 130, 246, 25);
        let mut path = BezPath::new();
        path.move_to(Point::new(rect.x0, rect.y0));
        path.line_to(Point::new(rect.x1, rect.y0));
        path.line_to(Point::new(rect.x1, rect.y1));
        path.line_to(Point::new(rect.x0, rect.y1));
        path.close_path();

        self.scene
            .fill(Fill::NonZero, transform, fill_color, None, &path);

        let stroke_width = 1.0 / self.zoom;
        let dash_len = 4.0 / self.zoom;
        let stroke = Stroke::new(stroke_width).with_dashes(0.0, [dash_len, dash_len]);
        self.scene
            .stroke(&stroke, transform, self.selection_color, None, &path);
    }
}
