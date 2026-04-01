use super::super::effects::{apply_hand_drawn_effect, generate_fill_pattern};
use crate::vello_impl::VelloRenderer;
use astra_canvas::shapes::{FillPattern, StrokeStyle};
use kurbo::{Affine, BezPath, Shape as KurboShape, Stroke};
use peniko::{Color, Fill};

impl VelloRenderer {
    #[allow(clippy::too_many_arguments)]
    fn fill_path_with_pattern(
        &mut self,
        fill_path: &BezPath,
        bounds_path: &BezPath,
        fill_color: Color,
        fill_pattern: FillPattern,
        stroke_width: f64,
        seed: u32,
        transform: Affine,
    ) {
        match fill_pattern {
            FillPattern::Solid => {
                self.scene
                    .fill(Fill::NonZero, transform, fill_color, None, fill_path);
            }
            _ => {
                let rgba = fill_color.to_rgba8();
                let bg_color = Color::from_rgba8(
                    rgba.r,
                    rgba.g,
                    rgba.b,
                    (rgba.a as f32 * 0.15) as u8,
                );
                self.scene
                    .fill(Fill::NonZero, transform, bg_color, None, fill_path);

                let bounds = bounds_path.bounding_box();
                let pattern_path =
                    generate_fill_pattern(fill_pattern, bounds, stroke_width, seed);

                self.scene
                    .push_clip_layer(Fill::NonZero, transform, fill_path);
                let pattern_stroke = Stroke::new(stroke_width * 0.5);
                self.scene
                    .stroke(&pattern_stroke, transform, fill_color, None, &pattern_path);
                self.scene.pop_layer();
            }
        }
    }

    pub(crate) fn render_path_cached(
        &mut self,
        shape_id: &str,
        path: &BezPath,
        style: &astra_canvas::shapes::ShapeStyle,
        transform: Affine,
    ) {
        let roughness = style.sloppiness.roughness();
        let seed = style.seed;

        if let Some(fill_color) = style.fill_with_opacity() {
            let fill_path = if roughness > 0.0 {
                self.get_cached_hand_drawn(shape_id, path, roughness * 0.3, seed, 0)
            } else {
                path.clone()
            };

            self.fill_path_with_pattern(
                &fill_path,
                path,
                fill_color,
                style.fill_pattern,
                style.stroke_width,
                seed,
                transform,
            );
        }

        if roughness > 0.0 {
            let stroke = Stroke::new(style.stroke_width);
            let path1 = self.get_cached_hand_drawn(shape_id, path, roughness, seed, 0);
            self.scene.stroke(
                &stroke,
                transform,
                style.stroke_with_opacity(),
                None,
                &path1,
            );
            let path2 = self.get_cached_hand_drawn(shape_id, path, roughness, seed, 1);
            self.scene.stroke(
                &stroke,
                transform,
                style.stroke_with_opacity(),
                None,
                &path2,
            );
        } else {
            let stroke = Stroke::new(style.stroke_width);
            self.scene
                .stroke(&stroke, transform, style.stroke_with_opacity(), None, path);
        }
    }

    pub(crate) fn render_stroke_only(
        &mut self,
        path: &BezPath,
        style: &astra_canvas::shapes::ShapeStyle,
        stroke_style: StrokeStyle,
        transform: Affine,
    ) {
        let roughness = style.sloppiness.roughness();
        let seed = style.seed;

        let mut stroke = Stroke::new(style.stroke_width);
        match stroke_style {
            StrokeStyle::Solid => {}
            StrokeStyle::Dashed => {
                let dash_len = style.stroke_width * 4.0;
                let gap_len = style.stroke_width * 2.0;
                stroke = stroke.with_dashes(0.0, [dash_len, gap_len]);
            }
            StrokeStyle::Dotted => {
                let dot_len = style.stroke_width;
                let gap_len = style.stroke_width * 2.0;
                stroke = stroke.with_dashes(0.0, [dot_len, gap_len]);
            }
        }

        if roughness > 0.0 {
            let path1 = apply_hand_drawn_effect(path, roughness, self.zoom, seed, 0);
            self.scene.stroke(
                &stroke,
                transform,
                style.stroke_with_opacity(),
                None,
                &path1,
            );
            let path2 = apply_hand_drawn_effect(path, roughness, self.zoom, seed, 1);
            self.scene.stroke(
                &stroke,
                transform,
                style.stroke_with_opacity(),
                None,
                &path2,
            );
        } else {
            self.scene
                .stroke(&stroke, transform, style.stroke_with_opacity(), None, path);
        }
    }

    pub(crate) fn render_filled_with_stroke_style(
        &mut self,
        shape_id: &str,
        path: &BezPath,
        style: &astra_canvas::shapes::ShapeStyle,
        stroke_style: StrokeStyle,
        transform: Affine,
    ) {
        let roughness = style.sloppiness.roughness();
        let seed = style.seed;

        if let Some(fill_color) = style.fill_with_opacity() {
            let fill_path = if roughness > 0.0 {
                self.get_cached_hand_drawn(shape_id, path, roughness * 0.3, seed, 0)
            } else {
                path.clone()
            };

            self.fill_path_with_pattern(
                &fill_path,
                path,
                fill_color,
                style.fill_pattern,
                style.stroke_width,
                seed,
                transform,
            );
        }

        self.render_stroke_only(path, style, stroke_style, transform);
    }
}
