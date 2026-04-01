use super::format::ImageFormat;
use crate::shapes::{ShapeId, ShapeStyle};
use kurbo::{Point, Rect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub(crate) id: ShapeId,
    pub position: Point,
    pub width: f64,
    pub height: f64,
    pub source_width: u32,
    pub source_height: u32,
    pub format: ImageFormat,
    pub data_base64: String,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default)]
    pub corner_radius: f64,
    pub style: ShapeStyle,
}

impl Image {
    pub const DEFAULT_MAX_SIZE: f64 = 800.0;

    pub fn new(
        position: Point,
        data: &[u8],
        source_width: u32,
        source_height: u32,
        format: ImageFormat,
    ) -> Self {
        use base64::{Engine, engine::general_purpose::STANDARD};

        Self {
            id: Uuid::new_v4(),
            position,
            width: source_width as f64,
            height: source_height as f64,
            source_width,
            source_height,
            format,
            data_base64: STANDARD.encode(data),
            rotation: 0.0,
            corner_radius: 0.0,
            style: ShapeStyle::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn reconstruct(
        id: ShapeId,
        position: Point,
        width: f64,
        height: f64,
        source_width: u32,
        source_height: u32,
        format: ImageFormat,
        data_base64: String,
        rotation: f64,
        corner_radius: f64,
        style: ShapeStyle,
    ) -> Self {
        Self {
            id,
            position,
            width,
            height,
            source_width,
            source_height,
            format,
            data_base64,
            rotation,
            corner_radius,
            style,
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn centered_at(mut self, center: Point) -> Self {
        self.position = Point::new(center.x - self.width / 2.0, center.y - self.height / 2.0);
        self
    }

    pub fn fit_within_default_size(self) -> Self {
        self.fit_within(Self::DEFAULT_MAX_SIZE, Self::DEFAULT_MAX_SIZE)
    }

    pub fn fitted_to_default_size(mut self) -> Self {
        if self.source_width as f64 > Self::DEFAULT_MAX_SIZE
            || self.source_height as f64 > Self::DEFAULT_MAX_SIZE
        {
            self = self.fit_within_default_size();
        }
        self
    }

    pub fn new_centered(
        center: Point,
        data: &[u8],
        source_width: u32,
        source_height: u32,
        format: ImageFormat,
    ) -> Self {
        Self::new(Point::ZERO, data, source_width, source_height, format)
            .fitted_to_default_size()
            .centered_at(center)
    }

    pub fn fit_within(mut self, max_width: f64, max_height: f64) -> Self {
        let aspect = self.source_width as f64 / self.source_height as f64;
        let target_aspect = max_width / max_height;

        if aspect > target_aspect {
            self.width = max_width;
            self.height = max_width / aspect;
        } else {
            self.height = max_height;
            self.width = max_height * aspect;
        }

        self
    }

    pub fn data(&self) -> Option<Vec<u8>> {
        use base64::{Engine, engine::general_purpose::STANDARD};
        STANDARD.decode(&self.data_base64).ok()
    }

    pub fn as_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.position.x + self.width,
            self.position.y + self.height,
        )
    }

    pub fn data_size(&self) -> usize {
        self.data_base64.len() * 3 / 4
    }
}
