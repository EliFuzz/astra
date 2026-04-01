use super::UiState;
use astra_canvas::shapes::{FontFamily, FontWeight, Shape};
use astra_canvas::tools::ToolKind;

#[derive(Debug, Clone, Default)]
pub struct SelectedShapeProps {
    pub has_selection: bool,
    pub selection_count: usize,
    pub is_text: bool,
    pub is_math: bool,
    pub is_rectangle: bool,
    pub is_diamond: bool,
    pub is_ellipse: bool,
    pub is_image: bool,
    pub is_line: bool,
    pub is_arrow: bool,
    pub is_freehand: bool,
    pub font_size: f32,
    pub font_family: FontFamily,
    pub font_weight: FontWeight,
    pub corner_radius: f32,
    pub path_style: u8,
    pub stroke_style: u8,
    pub sloppiness: u8,
    pub fill_pattern: u8,
    pub has_fill: bool,
    pub is_drawing_tool: bool,
    pub tool_is_rectangle: bool,
    pub tool_is_diamond: bool,
    pub tool_is_ellipse: bool,
    pub opacity: f32,
}

impl SelectedShapeProps {
    pub fn from_shape(shape: &Shape) -> Self {
        Self::from_shape_with_count(shape, 1)
    }

    pub fn from_shape_with_count(shape: &Shape, count: usize) -> Self {
        let sloppiness = shape.style().sloppiness as u8;
        let fill_pattern = shape.style().fill_pattern as u8;
        let has_fill = shape.style().fill_color.is_some();
        let opacity = shape.style().opacity as f32;

        match shape {
            Shape::Text(text) => Self {
                has_selection: true,
                selection_count: count,
                is_text: true,
                font_size: text.font_size as f32,
                font_family: text.font_family,
                font_weight: text.font_weight,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Rectangle(rect) => Self {
                has_selection: true,
                selection_count: count,
                is_rectangle: true,
                corner_radius: rect.corner_radius as f32,
                stroke_style: rect.stroke_style as u8,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Diamond(diamond) => Self {
                has_selection: true,
                selection_count: count,
                is_diamond: true,
                corner_radius: diamond.corner_radius as f32,
                stroke_style: diamond.stroke_style as u8,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Ellipse(ellipse) => Self {
                has_selection: true,
                selection_count: count,
                is_ellipse: true,
                stroke_style: ellipse.stroke_style as u8,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Image(image) => Self {
                has_selection: true,
                selection_count: count,
                is_image: true,
                corner_radius: image.corner_radius as f32,
                opacity,
                ..Default::default()
            },
            Shape::Line(line) => Self {
                has_selection: true,
                selection_count: count,
                is_line: true,
                path_style: line.path_style as u8,
                stroke_style: line.stroke_style as u8,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Arrow(arrow) => Self {
                has_selection: true,
                selection_count: count,
                is_arrow: true,
                path_style: arrow.path_style as u8,
                stroke_style: arrow.stroke_style as u8,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Freehand(_) => Self {
                has_selection: true,
                selection_count: count,
                is_freehand: true,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Math(math) => Self {
                has_selection: true,
                selection_count: count,
                is_math: true,
                font_size: math.font_size as f32,
                sloppiness,
                fill_pattern,
                has_fill,
                opacity,
                ..Default::default()
            },
            Shape::Group(group) => {
                let mut props = Self {
                    has_selection: true,
                    selection_count: count,
                    sloppiness,
                    fill_pattern,
                    has_fill,
                    opacity,
                    ..Default::default()
                };
                for child in &group.children {
                    let child_props = Self::from_shape(child);
                    props.is_text = props.is_text || child_props.is_text;
                    props.is_rectangle = props.is_rectangle || child_props.is_rectangle;
                    props.is_diamond = props.is_diamond || child_props.is_diamond;
                    props.is_ellipse = props.is_ellipse || child_props.is_ellipse;
                    props.is_line = props.is_line || child_props.is_line;
                    props.is_arrow = props.is_arrow || child_props.is_arrow;
                    props.is_freehand = props.is_freehand || child_props.is_freehand;
                    props.is_image = props.is_image || child_props.is_image;
                    if child_props.is_text {
                        props.font_size = child_props.font_size;
                        props.font_family = child_props.font_family;
                        props.font_weight = child_props.font_weight;
                    }
                    if child_props.is_rectangle || child_props.is_diamond {
                        props.corner_radius = child_props.corner_radius;
                        props.stroke_style = child_props.stroke_style;
                    }
                    if child_props.is_ellipse {
                        props.stroke_style = child_props.stroke_style;
                    }
                    if child_props.has_fill {
                        props.has_fill = true;
                    }
                }
                props
            },
        }
    }

    pub fn for_tool(tool: ToolKind, ui_state: &UiState) -> Self {
        Self {
            is_drawing_tool: true,
            tool_is_rectangle: tool == ToolKind::Rectangle,
            tool_is_diamond: tool == ToolKind::Diamond,
            tool_is_ellipse: tool == ToolKind::Ellipse,
            is_line: tool == ToolKind::Line,
            is_arrow: tool == ToolKind::Arrow,
            is_freehand: tool == ToolKind::Freehand || tool == ToolKind::Highlighter,
            sloppiness: ui_state.sloppiness as u8,
            fill_pattern: ui_state.fill_pattern as u8,
            has_fill: ui_state.fill_color.is_some(),
            path_style: ui_state.path_style,
            corner_radius: ui_state.corner_radius,
            ..Default::default()
        }
    }

    pub fn shows_stroke_color(&self) -> bool {
        !self.is_image
    }

    pub fn shows_fill_color(&self) -> bool {
        self.is_rectangle
            || self.is_diamond
            || self.is_ellipse
            || self.tool_is_rectangle
            || self.tool_is_diamond
            || self.tool_is_ellipse
    }

    pub fn shows_stroke_width(&self) -> bool {
        !self.is_text && !self.is_image
    }

    pub fn shows_stroke_style(&self) -> bool {
        self.is_rectangle
            || self.is_diamond
            || self.is_ellipse
            || self.is_line
            || self.is_arrow
            || self.tool_is_rectangle
            || self.tool_is_diamond
            || self.tool_is_ellipse
    }

    pub fn shows_sloppiness(&self) -> bool {
        !self.is_text && !self.is_freehand && !self.is_image
    }

    pub fn shows_edges(&self) -> bool {
        self.is_rectangle || self.is_diamond || self.is_image || self.tool_is_rectangle || self.tool_is_diamond
    }

    pub fn shows_path_style(&self) -> bool {
        self.is_line || self.is_arrow
    }
}
