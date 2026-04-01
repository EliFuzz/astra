use super::document::CanvasDocument;
use crate::camera::Camera;
use crate::shapes::{Shape, ShapeId};
use crate::tools::{ToolKind, ToolManager};
use crate::interaction::{EditingKind, WidgetManager, WidgetState};
use kurbo::{Affine, Point, Rect, Vec2};

#[derive(Debug, Clone)]
pub struct Canvas {
    pub document: CanvasDocument,
    pub camera: Camera,
    pub tool_manager: ToolManager,
    pub selection: Vec<ShapeId>,
    pub viewport_size: kurbo::Size,
    pub widgets: WidgetManager,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            document: CanvasDocument::new(),
            camera: Camera::new(),
            tool_manager: ToolManager::new(),
            selection: Vec::new(),
            viewport_size: kurbo::Size::new(800.0, 600.0),
            widgets: WidgetManager::new(),
        }
    }

    pub fn with_document(document: CanvasDocument) -> Self {
        Self {
            document,
            camera: Camera::new(),
            tool_manager: ToolManager::new(),
            selection: Vec::new(),
            viewport_size: kurbo::Size::new(800.0, 600.0),
            widgets: WidgetManager::new(),
        }
    }

    pub fn set_viewport_size(&mut self, width: f64, height: f64) {
        self.viewport_size = kurbo::Size::new(width, height);
    }

    pub fn visible_world_bounds(&self) -> kurbo::Rect {
        let top_left = self.camera.screen_to_world(kurbo::Point::ZERO);
        let bottom_right = self.camera.screen_to_world(kurbo::Point::new(
            self.viewport_size.width,
            self.viewport_size.height,
        ));
        kurbo::Rect::new(top_left.x, top_left.y, bottom_right.x, bottom_right.y)
    }

    pub fn select(&mut self, id: ShapeId) {
        self.clear_selection();
        self.add_to_selection(id);
    }

    pub fn add_to_selection(&mut self, id: ShapeId) {
        if !self.selection.contains(&id) {
            self.selection.push(id);
        }
        self.widgets.add_to_selection(id);
    }

    pub fn clear_selection(&mut self) {
        self.selection.clear();
        self.widgets.clear_selection();
    }

    pub fn select_all(&mut self) {
        self.clear_selection();
        for &id in &self.document.z_order {
            self.selection.push(id);
            self.widgets.add_to_selection(id);
        }
    }

    pub fn is_selected(&self, id: ShapeId) -> bool {
        self.widgets.is_selected(id)
    }

    pub fn enter_text_editing(&mut self, id: ShapeId) {
        self.widgets.enter_editing(id, EditingKind::Text);
    }

    pub fn exit_text_editing(&mut self) {
        self.widgets.exit_editing();
    }

    pub fn editing_shape(&self) -> Option<ShapeId> {
        self.widgets.focused()
    }

    pub fn is_editing(&self, id: ShapeId) -> bool {
        self.widgets.is_editing_shape(id)
    }

    pub fn widget_state(&self, id: ShapeId) -> WidgetState {
        self.widgets.state(id)
    }

    pub fn set_tool(&mut self, tool: ToolKind) {
        self.tool_manager.set_tool(tool);
    }

    pub fn replace_document(&mut self, mut document: CanvasDocument) {
        if !document.shapes.is_empty() && document.spatial_index.is_empty() {
            document.spatial_index.rebuild(&document.shapes);
        }
        self.document = document;
        self.clear_selection();
    }

    pub fn replace_document_and_reset_camera(&mut self, document: CanvasDocument) {
        self.replace_document(document);
        self.camera.reset();
    }

    pub fn insert_shape_and_select(&mut self, shape: Shape) -> ShapeId {
        self.document.push_undo();
        self.clear_selection();
        let id = shape.id();
        self.document.add_shape(shape);
        self.add_to_selection(id);
        id
    }

    pub fn insert_shapes_and_select<I>(&mut self, shapes: I) -> Vec<ShapeId>
    where
        I: IntoIterator<Item = Shape>,
    {
        let shapes: Vec<Shape> = shapes.into_iter().collect();
        if shapes.is_empty() {
            return Vec::new();
        }

        self.document.push_undo();
        self.clear_selection();

        let mut ids = Vec::with_capacity(shapes.len());
        for shape in shapes {
            let id = shape.id();
            self.document.add_shape(shape);
            self.add_to_selection(id);
            ids.push(id);
        }
        ids
    }

    pub fn insert_shapes_centered_at(&mut self, shapes: Vec<Shape>, center: Point) -> Vec<ShapeId> {
        let Some(bounds) = union_bounds(&shapes) else {
            return Vec::new();
        };
        let offset = Vec2::new(
            center.x - (bounds.x0 + bounds.x1) / 2.0,
            center.y - (bounds.y0 + bounds.y1) / 2.0,
        );
        self.insert_shapes_and_select(shapes.into_iter().map(|mut shape| {
            shape.transform(Affine::translate(offset));
            shape
        }))
    }

    pub fn fit_to_content(&mut self) {
        if let Some(bounds) = self.document.bounds() {
            self.camera.fit_to_bounds(bounds, self.viewport_size, 50.0);
        }
    }
}

fn union_bounds(shapes: &[Shape]) -> Option<Rect> {
    shapes
        .iter()
        .map(Shape::bounds)
        .reduce(|acc, bounds| acc.union(bounds))
}
