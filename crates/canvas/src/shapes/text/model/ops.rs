use super::super::font_family::FontFamily;
use super::super::font_weight::FontWeight;
use super::Text;
use crate::shapes::{SerializableColor, ShapeId, ShapeStyle};
use kurbo::Point;
use std::sync::RwLock;
use uuid::Uuid;

impl Text {
    pub const DEFAULT_FONT_SIZE: f64 = 20.0;

    pub fn new(position: Point, content: String) -> Self {
        let char_count = content.chars().count();
        Self {
            id: Uuid::new_v4(),
            position,
            content,
            font_size: Self::DEFAULT_FONT_SIZE,
            font_family: FontFamily::default(),
            font_weight: FontWeight::default(),
            rotation: 0.0,
            style: ShapeStyle::default(),
            char_colors: vec![None; char_count],
            text_align: 0,
            cached_size: RwLock::new(None),
        }
    }

    pub fn set_cached_size(&self, width: f64, height: f64) {
        if let Ok(mut cache) = self.cached_size.write() {
            *cache = Some((width, height));
        }
    }

    pub fn invalidate_cache(&self) {
        if let Ok(mut cache) = self.cached_size.write() {
            *cache = None;
        }
    }

    pub fn apply_color_to_range(
        &mut self,
        start_char: usize,
        end_char: usize,
        color: SerializableColor,
    ) {
        let char_count = self.content.chars().count();
        self.char_colors.resize(char_count, None);

        for i in start_char..end_char.min(char_count) {
            self.char_colors[i] = Some(color);
        }
    }

    pub fn sync_char_colors_after_edit(&mut self, edit_char_pos: usize, old_char_count: usize) {
        let new_char_count = self.content.chars().count();
        if new_char_count == old_char_count {
            return;
        }

        if new_char_count > old_char_count {
            let inserted = new_char_count - old_char_count;
            let insert_pos = edit_char_pos.min(self.char_colors.len());
            for _ in 0..inserted {
                if insert_pos <= self.char_colors.len() {
                    self.char_colors.insert(insert_pos, None);
                } else {
                    self.char_colors.push(None);
                }
            }
        } else {
            let deleted = old_char_count - new_char_count;
            let delete_pos = edit_char_pos.min(self.char_colors.len());
            for _ in 0..deleted {
                if delete_pos < self.char_colors.len() {
                    self.char_colors.remove(delete_pos);
                }
            }
        }
        self.char_colors.resize(new_char_count, None);
    }

    pub fn sync_char_colors_with_content(&mut self) {
        let char_count = self.content.chars().count();
        self.char_colors.resize(char_count, None);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: ShapeId,
        position: Point,
        content: String,
        font_size: f64,
        font_family: FontFamily,
        font_weight: FontWeight,
        rotation: f64,
        style: ShapeStyle,
        char_colors: Vec<Option<SerializableColor>>,
    ) -> Self {
        Self {
            id,
            position,
            content,
            font_size,
            font_family,
            font_weight,
            rotation,
            style,
            char_colors,
            text_align: 0,
            cached_size: RwLock::new(None),
        }
    }

    pub fn with_font_size(mut self, size: f64) -> Self {
        self.font_size = size;
        self
    }

    pub fn with_font_family(mut self, family: FontFamily) -> Self {
        self.font_family = family;
        self
    }

    pub fn with_font_weight(mut self, weight: FontWeight) -> Self {
        self.font_weight = weight;
        self
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.invalidate_cache();
    }

    pub fn fit_within_bounds(&mut self, parent_bounds: kurbo::Rect) {
        let padding = 8.0;
        let max_w = (parent_bounds.width() - padding).max(1.0);
        let max_h = (parent_bounds.height() - padding).max(1.0);
        let center = parent_bounds.center();

        self.invalidate_cache();
        let tw = self.approximate_width();
        let th = self.approximate_height();
        if tw > max_w || th > max_h {
            let shrink = (max_w / tw.max(1.0)).min(max_h / th.max(1.0));
            self.font_size *= shrink;
            self.invalidate_cache();
        }
        let tw = self.approximate_width().max(20.0);
        let th = self.approximate_height();
        self.position = Point::new(center.x - tw / 2.0, center.y - th / 2.0);
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub(crate) fn approximate_width(&self) -> f64 {
        let max_line_len = self
            .content
            .lines()
            .map(|line| line.len())
            .max()
            .unwrap_or(0);

        let char_width_factor = match (&self.font_family, &self.font_weight) {
            (FontFamily::Handwriting, FontWeight::Light) => 0.50,
            (FontFamily::Handwriting, FontWeight::Regular) => 0.55,
            (FontFamily::Handwriting, FontWeight::Heavy) => 0.60,
            (FontFamily::Sans, FontWeight::Light) => 0.50,
            (FontFamily::Sans, FontWeight::Regular) => 0.52,
            (FontFamily::Sans, FontWeight::Heavy) => 0.55,
        };

        max_line_len as f64 * self.font_size * char_width_factor
    }

    pub(crate) fn approximate_height(&self) -> f64 {
        let line_count = self.content.lines().count().max(1);
        let line_count = if self.content.ends_with('\n') {
            line_count + 1
        } else {
            line_count
        };
        line_count as f64 * self.font_size * 1.2
    }
}
