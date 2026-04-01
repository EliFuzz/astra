use astra_canvas::canvas::Canvas;
use astra_canvas::shapes::Shape;
use astra_canvas::snap::SmartGuide;
use kurbo::{Affine, Rect, Size};
use peniko::Color;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Initialization failed: {0}")]
    InitFailed(String),
    #[error("Render failed: {0}")]
    RenderFailed(String),
    #[error("Surface error: {0}")]
    Surface(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GridStyle {
    #[default]
    None,
    Lines,
    HorizontalLines,
    CrossPlus,
    Dots,
}

impl From<astra_core::PreferredGridStyle> for GridStyle {
    fn from(pref: astra_core::PreferredGridStyle) -> Self {
        match pref {
            astra_core::PreferredGridStyle::None => Self::None,
            astra_core::PreferredGridStyle::Lines => Self::Lines,
            astra_core::PreferredGridStyle::HorizontalLines => Self::HorizontalLines,
            astra_core::PreferredGridStyle::CrossPlus => Self::CrossPlus,
            astra_core::PreferredGridStyle::Dots => Self::Dots,
        }
    }
}

impl From<GridStyle> for astra_core::PreferredGridStyle {
    fn from(style: GridStyle) -> Self {
        match style {
            GridStyle::None => Self::None,
            GridStyle::Lines => Self::Lines,
            GridStyle::HorizontalLines => Self::HorizontalLines,
            GridStyle::CrossPlus => Self::CrossPlus,
            GridStyle::Dots => Self::Dots,
        }
    }
}

impl GridStyle {
    pub fn next(self) -> Self {
        match self {
            GridStyle::None => GridStyle::Lines,
            GridStyle::Lines => GridStyle::HorizontalLines,
            GridStyle::HorizontalLines => GridStyle::CrossPlus,
            GridStyle::CrossPlus => GridStyle::Dots,
            GridStyle::Dots => GridStyle::None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            GridStyle::None => "None",
            GridStyle::Lines => "Grid",
            GridStyle::HorizontalLines => "Lines",
            GridStyle::CrossPlus => "Crosses",
            GridStyle::Dots => "Dots",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AngleSnapInfo {
    pub start_point: kurbo::Point,
    pub end_point: kurbo::Point,
    pub angle_degrees: f64,
    pub is_snapped: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct RotationInfo {
    pub center: kurbo::Point,
    pub angle: f64,
    pub snapped: bool,
}

pub struct RenderContext<'a> {
    pub canvas: &'a Canvas,
    pub viewport_size: Size,
    pub scale_factor: f64,
    pub background_color: Color,
    pub grid_style: GridStyle,
    pub selection_color: Color,
    pub selection_rect: Option<Rect>,
    pub editing_shape_id: Option<astra_canvas::shapes::ShapeId>,
    pub snap_point: Option<kurbo::Point>,
    pub angle_snap_info: Option<AngleSnapInfo>,
    pub rotation_info: Option<RotationInfo>,
    pub smart_guides: Vec<SmartGuide>,
    pub eraser_cursor: Option<(kurbo::Point, f64)>,
    pub laser_pointer: Option<(kurbo::Point, Vec<(kurbo::Point, f64)>)>,
    pub arrow_snap_targets: Vec<kurbo::Point>,
}

impl<'a> RenderContext<'a> {
    pub fn new(canvas: &'a Canvas, viewport_size: Size) -> Self {
        Self {
            canvas,
            viewport_size,
            scale_factor: 1.0,
            background_color: Color::from_rgba8(18, 18, 24, 255),
            grid_style: GridStyle::None,
            selection_color: Color::from_rgba8(59, 130, 246, 255),
            selection_rect: None,
            editing_shape_id: None,
            snap_point: None,
            angle_snap_info: None,
            rotation_info: None,
            smart_guides: Vec::new(),
            eraser_cursor: None,
            laser_pointer: None,
            arrow_snap_targets: Vec::new(),
        }
    }

    pub fn with_scale_factor(mut self, scale_factor: f64) -> Self {
        self.scale_factor = scale_factor;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_grid(mut self, style: GridStyle) -> Self {
        self.grid_style = style;
        self
    }

    pub fn with_selection_rect(mut self, rect: Option<Rect>) -> Self {
        self.selection_rect = rect;
        self
    }

    pub fn with_editing_shape(mut self, shape_id: Option<astra_canvas::shapes::ShapeId>) -> Self {
        self.editing_shape_id = shape_id;
        self
    }

    pub fn with_snap_point(mut self, point: Option<kurbo::Point>) -> Self {
        self.snap_point = point;
        self
    }

    pub fn with_angle_snap(mut self, info: Option<AngleSnapInfo>) -> Self {
        self.angle_snap_info = info;
        self
    }

    pub fn with_rotation_info(mut self, info: Option<RotationInfo>) -> Self {
        self.rotation_info = info;
        self
    }

    pub fn with_smart_guides(mut self, guides: Vec<SmartGuide>) -> Self {
        self.smart_guides = guides;
        self
    }

    pub fn with_eraser_cursor(mut self, cursor: Option<(kurbo::Point, f64)>) -> Self {
        self.eraser_cursor = cursor;
        self
    }

    pub fn with_laser_pointer(
        mut self,
        pointer: Option<(kurbo::Point, Vec<(kurbo::Point, f64)>)>,
    ) -> Self {
        self.laser_pointer = pointer;
        self
    }

    pub fn with_arrow_snap_targets(mut self, targets: Vec<kurbo::Point>) -> Self {
        self.arrow_snap_targets = targets;
        self
    }
}

pub trait Renderer: Send + Sync {
    fn build_scene(&mut self, ctx: &RenderContext);

    fn background_color(&self, ctx: &RenderContext) -> Color {
        ctx.background_color
    }
}

pub trait ShapeRenderer {
    fn render_shape(&mut self, shape: &Shape, transform: Affine, selected: bool);
}
