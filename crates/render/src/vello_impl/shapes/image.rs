use crate::vello_impl::VelloRenderer;
use astra_canvas::shapes::ShapeTrait;
use kurbo::{Affine, BezPath, Point, RoundedRect, Shape as KurboShape, Stroke};
use peniko::{BlendMode, Color, Compose, Fill, Mix};

impl VelloRenderer {
    pub(crate) fn render_image(&mut self, image: &astra_canvas::shapes::Image, transform: Affine) {
        use std::sync::Arc;

        let id_str = image.id().to_string();

        let image_data = if let Some(cached) = self.image_cache.get(&id_str) {
            cached.clone()
        } else if let Some(raw_data) = image.data() {
            if let Ok(decoded) = ::image::load_from_memory(&raw_data) {
                let rgba = decoded.to_rgba8();
                let (width, height) = rgba.dimensions();
                let blob = peniko::Blob::new(Arc::new(rgba.into_vec()));
                let img_data = peniko::ImageData {
                    data: blob,
                    format: peniko::ImageFormat::Rgba8,
                    width,
                    height,
                    alpha_type: peniko::ImageAlphaType::Alpha,
                };
                self.image_cache.insert(id_str.clone(), img_data.clone());
                img_data
            } else {
                self.render_image_placeholder(image, transform);
                return;
            }
        } else {
            self.render_image_placeholder(image, transform);
            return;
        };

        let bounds = image.bounds();
        let scale_x = bounds.width() / image_data.width as f64;
        let scale_y = bounds.height() / image_data.height as f64;

        let image_transform = transform
            * Affine::translate((bounds.x0, bounds.y0))
            * Affine::scale_non_uniform(scale_x, scale_y);

        let opacity = image.style.opacity;
        let needs_clip = image.corner_radius > 0.0;
        let needs_opacity = opacity < 1.0;

        if needs_clip {
            let clip_path = RoundedRect::from_rect(bounds, image.corner_radius).to_path(0.1);
            self.scene
                .push_clip_layer(Fill::NonZero, transform, &clip_path);
        }

        if needs_opacity {
            let blend = BlendMode::new(Mix::Normal, Compose::SrcOver);
            self.scene.push_layer(
                Fill::NonZero,
                blend,
                opacity as f32,
                transform,
                &bounds,
            );
        }

        self.scene.draw_image(&image_data, image_transform);

        if needs_opacity {
            self.scene.pop_layer();
        }

        if needs_clip {
            self.scene.pop_layer();
        }
    }

    fn render_image_placeholder(&mut self, image: &astra_canvas::shapes::Image, transform: Affine) {
        let bounds = image.bounds();
        let rect_path = bounds.to_path(0.1);
        self.scene.fill(
            Fill::NonZero,
            transform,
            Color::from_rgba8(200, 200, 200, 255),
            None,
            &rect_path,
        );

        let stroke = Stroke::new(2.0);
        let mut x_path = BezPath::new();
        x_path.move_to(Point::new(bounds.x0, bounds.y0));
        x_path.line_to(Point::new(bounds.x1, bounds.y1));
        x_path.move_to(Point::new(bounds.x1, bounds.y0));
        x_path.line_to(Point::new(bounds.x0, bounds.y1));
        self.scene.stroke(
            &stroke,
            transform,
            Color::from_rgba8(150, 150, 150, 255),
            None,
            &x_path,
        );

        self.scene.stroke(
            &stroke,
            transform,
            Color::from_rgba8(100, 100, 100, 255),
            None,
            &rect_path,
        );
    }
}
