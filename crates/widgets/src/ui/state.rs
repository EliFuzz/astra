use crate::COLORS;
use astra_canvas::shapes::{FillPattern, ShapeId, ShapeStyle, Sloppiness};
use astra_canvas::tools::ToolKind;
use astra_render::GridStyle;
use egui::Color32;

#[derive(Clone, Copy, PartialEq)]
pub enum ColorPopover {
    None,
    StrokeFull,
    FillFull,
    BgFull,
}

pub struct UiState {
    pub current_tool: ToolKind,
    pub stroke_color: Color32,
    pub fill_color: Option<Color32>,
    pub stroke_width: f32,
    pub selection_count: usize,
    pub menu_open: bool,
    pub color_popover: ColorPopover,
    pub grid_style: GridStyle,
    pub zoom_level: f64,
    pub grid_snap_enabled: bool,
    pub smart_snap_enabled: bool,
    pub angle_snap_enabled: bool,
    pub export_scale: u8,
    pub sloppiness: Sloppiness,
    pub fill_pattern: FillPattern,
    pub corner_radius: f32,
    pub path_style: u8,
    pub bg_color: Color32,
    pub save_dialog_open: bool,
    pub open_dialog_open: bool,
    pub open_recent_dialog_open: bool,
    pub save_name_input: String,
    pub recent_documents: Vec<String>,
    pub selected_recent_document: Option<String>,
    pub clipboard_shapes: Option<String>,
    pub math_editor: Option<(ShapeId, String)>,
    pub font_size: f32,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_tool: ToolKind::Select,
            stroke_color: COLORS[11].shades[6],
            fill_color: None,
            stroke_width: 2.0,
            selection_count: 0,
            menu_open: false,
            color_popover: ColorPopover::None,
            grid_style: GridStyle::default(),
            zoom_level: astra_canvas::camera::BASE_ZOOM,
            grid_snap_enabled: false,
            smart_snap_enabled: false,
            angle_snap_enabled: false,
            export_scale: 2,
            sloppiness: Sloppiness::Artist,
            fill_pattern: FillPattern::Solid,
            corner_radius: 0.0,
            path_style: 0,
            bg_color: crate::theme::canvas_bg(),
            save_dialog_open: false,
            open_dialog_open: false,
            open_recent_dialog_open: false,
            save_name_input: String::new(),
            recent_documents: Vec::new(),
            selected_recent_document: None,
            clipboard_shapes: None,
            math_editor: None,
            font_size: 24.0,
        }
    }
}

impl UiState {
    pub fn has_open_overlay(&self) -> bool {
        self.color_popover != ColorPopover::None
            || self.menu_open
            || self.save_dialog_open
            || self.open_dialog_open
            || self.open_recent_dialog_open
    }

    pub fn close_overlays(&mut self) {
        self.color_popover = ColorPopover::None;
        self.menu_open = false;
        self.save_dialog_open = false;
        self.open_dialog_open = false;
        self.open_recent_dialog_open = false;
    }

    pub fn open_save_dialog(&mut self, name: impl Into<String>) {
        self.save_name_input = name.into();
        self.save_dialog_open = true;
    }

    pub fn open_document_dialog(&mut self, recent_only: bool) {
        if recent_only {
            self.open_recent_dialog_open = true;
        } else {
            self.open_dialog_open = true;
        }
    }

    pub fn set_recent_documents(&mut self, documents: Vec<String>) {
        self.recent_documents = documents;
    }

    pub fn remember_recent_document(&mut self, name: &str) {
        if self.recent_documents.iter().any(|entry| entry == name) {
            return;
        }

        self.recent_documents.insert(0, name.to_string());
        if self.recent_documents.len() > 10 {
            self.recent_documents.truncate(10);
        }
    }

    pub fn update_from_style(&mut self, style: &ShapeStyle) {
        let sc = style.stroke_color;
        self.stroke_color = Color32::from_rgba_unmultiplied(sc.r, sc.g, sc.b, sc.a);
        self.stroke_width = style.stroke_width as f32;
        self.fill_color = style
            .fill_color
            .map(|fc| Color32::from_rgba_unmultiplied(fc.r, fc.g, fc.b, fc.a));
        self.fill_pattern = style.fill_pattern;
        self.sloppiness = style.sloppiness;
    }

    pub fn to_shape_style(&self) -> ShapeStyle {
        use astra_canvas::shapes::SerializableColor;
        ShapeStyle {
            stroke_color: SerializableColor::new(
                self.stroke_color.r(),
                self.stroke_color.g(),
                self.stroke_color.b(),
                self.stroke_color.a(),
            ),
            stroke_width: self.stroke_width as f64,
            fill_color: self
                .fill_color
                .map(|c| SerializableColor::new(c.r(), c.g(), c.b(), c.a())),
            fill_pattern: self.fill_pattern,
            sloppiness: self.sloppiness,
            ..ShapeStyle::default()
        }
    }
}
