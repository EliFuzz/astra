use crate::renderer::ShapeRenderer;
use crate::vello_impl::VelloRenderer;
use astra_canvas::shapes::{Shape, StrokeStyle};
use kurbo::Affine;

impl VelloRenderer {
    fn render_closed_shape_with_stroke_style(
        &mut self,
        shape: &Shape,
        stroke_style: StrokeStyle,
        transform: Affine,
    ) {
        let path = shape.to_path();
        let shape_id = shape.id().to_string();
        if stroke_style == StrokeStyle::Solid {
            self.render_path_cached(&shape_id, &path, shape.style(), transform);
        } else {
            self.render_filled_with_stroke_style(
                &shape_id,
                &path,
                shape.style(),
                stroke_style,
                transform,
            );
        }
    }
}

impl ShapeRenderer for VelloRenderer {
    fn render_shape(&mut self, shape: &Shape, transform: Affine, selected: bool) {
        let rotation = shape.rotation();
        let shape_transform = if rotation.abs() > 0.001 {
            let center = shape.bounds().center();
            let center_vec = kurbo::Vec2::new(center.x, center.y);
            transform
                * Affine::translate(center_vec)
                * Affine::rotate(rotation)
                * Affine::translate(-center_vec)
        } else {
            transform
        };

        match shape {
            Shape::Text(text) => {
                self.render_text(text, shape_transform);
            }
            Shape::Group(group) => {
                for child in group.children() {
                    self.render_shape(child, shape_transform, false);
                }
            }
            Shape::Image(image) => {
                self.render_image(image, shape_transform);
            }
            Shape::Line(line) => {
                let path = shape.to_path();
                if line.closed {
                    self.render_path_cached(
                        &shape.id().to_string(),
                        &path,
                        shape.style(),
                        shape_transform,
                    );
                } else {
                    self.render_stroke_only(
                        &path,
                        shape.style(),
                        line.stroke_style,
                        shape_transform,
                    );
                }
            }
            Shape::Arrow(arrow) => {
                let path = shape.to_path();
                self.render_stroke_only(&path, shape.style(), arrow.stroke_style, shape_transform);
            }
            Shape::Freehand(freehand) => {
                if freehand.closed {
                    let path = shape.to_path();
                    self.render_path_cached(
                        &shape.id().to_string(),
                        &path,
                        shape.style(),
                        shape_transform,
                    );
                } else if freehand.has_pressure() {
                    self.render_freehand_with_pressure(freehand, shape_transform);
                } else {
                    let path = shape.to_path();
                    self.render_stroke_only(
                        &path,
                        shape.style(),
                        StrokeStyle::Solid,
                        shape_transform,
                    );
                }
            }
            Shape::Math(math) => {
                self.render_math(math, shape_transform);
            }
            Shape::Rectangle(rect) => {
                self.render_closed_shape_with_stroke_style(shape, rect.stroke_style, shape_transform);
            }
            Shape::Diamond(diamond) => {
                self.render_closed_shape_with_stroke_style(
                    shape,
                    diamond.stroke_style,
                    shape_transform,
                );
            }
            Shape::Ellipse(ellipse) => {
                self.render_closed_shape_with_stroke_style(
                    shape,
                    ellipse.stroke_style,
                    shape_transform,
                );
            }
        }

        if selected {
            self.render_shape_handles(shape, transform);
        }
    }
}
