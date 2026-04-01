use crate::canvas::Canvas;
use crate::selection::{
    Corner, HANDLE_HIT_TOLERANCE, HandleKind, get_handles, hit_test_boundary, hit_test_handles,
};
use crate::shapes::{Shape, ShapeId, ShapeTrait};
use crate::tools::ToolKind;
use kurbo::Point;

use super::EventHandler;

impl EventHandler {
    pub fn cancel(&mut self, canvas: &mut Canvas) {
        if let Some(manip) = self.manipulation.take() {
            if let Some(mut shape) = canvas.document.get_shape_mut(manip.shape_id) {
                *shape = manip.original_shape;
            }
        }
        if let Some(mm) = self.multi_move.take() {
            for (id, original) in mm.original_shapes {
                if let Some(mut shape) = canvas.document.get_shape_mut(id) {
                    *shape = original;
                }
            }
        }
        self.selection_rect = None;
        self.last_snap = None;
        self.last_angle_snap = None;
        self.rotation_state = None;
        canvas.tool_manager.cancel();
    }

    pub fn enter_text_edit(&mut self, canvas: &mut Canvas, id: ShapeId) {
        if let Some(shape) = canvas.document.get_shape_deep(id) {
            self.text_edit_anchor = get_handles(shape)
                .into_iter()
                .find(|h| matches!(h.kind, HandleKind::Corner(Corner::TopLeft)))
                .map(|h| h.position);
            let bounds = shape.bounds();
            self.text_edit_size = Some((bounds.width(), bounds.height()));
        }
        self.editing_text = Some(id);
        canvas.enter_text_editing(id);
    }

    pub fn exit_text_edit(&mut self, canvas: &mut Canvas) {
        let text_id = match self.editing_text {
            Some(id) => id,
            None => return,
        };
        let parent_shape_id = self.text_edit_parent_shape.take();

        let is_empty = canvas
            .document
            .get_shape_deep(text_id)
            .map(|s| matches!(s, Shape::Text(t) if t.content.trim().is_empty()))
            .unwrap_or(true);

        let is_in_group = find_parent_group(canvas, text_id).is_some();

        if is_empty && !is_in_group {
            canvas.document.remove_shape(text_id);
        } else if let Some(parent_id) = parent_shape_id {
            if canvas.document.get_shape(parent_id).is_some() {
                center_text_in_shape(canvas, text_id, parent_id);
                canvas.document.group_shapes(&[parent_id, text_id]);
                if let Some(group_id) = canvas.document.shapes.values().find_map(|s| {
                    if let Shape::Group(g) = s {
                        if g.children.iter().any(|c| c.id() == text_id) {
                            return Some(g.id);
                        }
                    }
                    None
                }) {
                    canvas.clear_selection();
                    canvas.select(group_id);
                }
            }
        } else if let Some(anchor) = self.text_edit_anchor {
            if let Some(group_id) = find_parent_group(canvas, text_id) {
                center_text_in_group(canvas, text_id, group_id);
                canvas.clear_selection();
                canvas.select(group_id);
            } else if let Some(mut guard) = canvas.document.get_shape_mut(text_id) {
                if let Shape::Text(text) = &mut *guard {
                    let bounds = text.bounds();
                    let half_w = bounds.width() / 2.0;
                    let half_h = bounds.height() / 2.0;
                    let rotation = text.rotation;
                    if rotation.abs() > 0.001 {
                        let cos_r = rotation.cos();
                        let sin_r = rotation.sin();
                        let rot_x = -half_w * cos_r + half_h * sin_r;
                        let rot_y = -half_w * sin_r - half_h * cos_r;
                        text.position =
                            Point::new(anchor.x - half_w - rot_x, anchor.y - half_h - rot_y);
                    } else {
                        text.position = anchor;
                    }
                }
            }
        }

        self.editing_text = None;
        self.text_edit_anchor = None;
        self.text_edit_size = None;
        canvas.exit_text_editing();
    }

    pub fn get_cursor_for_position(
        &self,
        canvas: &Canvas,
        world_point: Point,
    ) -> Option<Option<HandleKind>> {
        let handle_tolerance = HANDLE_HIT_TOLERANCE / canvas.camera.zoom;
        let boundary_tolerance = 8.0 / canvas.camera.zoom;

        match canvas.tool_manager.current_tool {
            ToolKind::Text => {
                let hits = canvas
                    .document
                    .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
                if let Some(&id) = hits.first() {
                    if let Some(shape @ Shape::Text(_)) = canvas.document.get_shape(id) {
                        if let Some(handle) = hit_test_handles(shape, world_point, handle_tolerance)
                        {
                            return Some(Some(handle));
                        }
                        if hit_test_boundary(shape, world_point, boundary_tolerance) {
                            return Some(None);
                        }
                    }
                }
            }
            ToolKind::Select => {
                for &shape_id in &canvas.selection {
                    if let Some(shape) = canvas.document.get_shape(shape_id) {
                        if let Some(handle) = hit_test_handles(shape, world_point, handle_tolerance)
                        {
                            return Some(Some(handle));
                        }
                    }
                }
                let hits = canvas
                    .document
                    .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
                if !hits.is_empty() {
                    return Some(None);
                }
            }
            _ => {}
        }
        None
    }
}

fn find_parent_group(canvas: &Canvas, text_id: ShapeId) -> Option<ShapeId> {
    canvas.document.shapes.values().find_map(|s| {
        if let Shape::Group(g) = s {
            if g.children.iter().any(|c| c.id() == text_id) {
                return Some(g.id);
            }
        }
        None
    })
}

fn center_text_in_shape(canvas: &mut Canvas, text_id: ShapeId, parent_id: ShapeId) {
    let parent_bounds = match canvas.document.get_shape(parent_id) {
        Some(shape) => shape.bounds(),
        None => return,
    };
    if let Some(mut guard) = canvas.document.get_shape_mut(text_id) {
        if let Shape::Text(text) = &mut *guard {
            text.fit_within_bounds(parent_bounds);
        }
    }
}

fn center_text_in_group(canvas: &mut Canvas, _text_id: ShapeId, group_id: ShapeId) {
    if let Some(Shape::Group(g)) = canvas.document.shapes.get_mut(&group_id) {
        g.fit_and_center_text_children();
    }
}

