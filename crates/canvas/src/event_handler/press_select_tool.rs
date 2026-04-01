use std::collections::HashMap;

use crate::canvas::Canvas;
use crate::input::InputState;
use crate::selection::{
    HANDLE_HIT_TOLERANCE, HandleKind, ManipulationState, MultiMoveState, hit_test_handles,
};
use crate::shapes::{Shape, Text};
use kurbo::Point;

use super::EventHandler;
use super::state::SelectionRect;

fn find_group_child_text(canvas: &Canvas, group_id: crate::shapes::ShapeId) -> Option<crate::shapes::ShapeId> {
    if let Some(Shape::Group(g)) = canvas.document.get_shape(group_id) {
        g.children.iter().find_map(|c| {
            if matches!(c, Shape::Text(_)) { Some(c.id()) } else { None }
        })
    } else {
        None
    }
}

pub(super) fn handle(
    handler: &mut EventHandler,
    canvas: &mut Canvas,
    world_point: Point,
    input: &InputState,
) {
    if input.is_double_click() {
        let hits = canvas
            .document
            .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
        if let Some(&id) = hits.first() {
            if let Some(Shape::Text(_)) = canvas.document.get_shape(id) {
                handler.enter_text_edit(canvas, id);
                canvas.clear_selection();
                canvas.select(id);
                return;
            }
            if let Some(Shape::Math(_)) = canvas.document.get_shape(id) {
                handler.pending_math_edit = Some(id);
                canvas.clear_selection();
                canvas.select(id);
                return;
            }
            if let Some(Shape::Group(_)) = canvas.document.get_shape(id) {
                if let Some(text_id) = find_group_child_text(canvas, id) {
                    canvas.clear_selection();
                    canvas.select(id);
                    handler.enter_text_edit(canvas, text_id);
                    return;
                }
            }
            if matches!(
                canvas.document.get_shape(id),
                Some(Shape::Rectangle(_) | Shape::Diamond(_) | Shape::Ellipse(_))
            ) {
                let bounds_center = canvas.document.get_shape(id).unwrap().bounds().center();
                let mut text = Text::new(bounds_center, String::new());
                text.style = canvas.tool_manager.current_style.clone();
                text.text_align = 1;
                let text_shape = Shape::Text(text);
                let text_id = text_shape.id();
                canvas.document.push_undo();
                canvas.document.add_shape(text_shape);
                canvas.clear_selection();
                canvas.select(text_id);
                handler.text_edit_parent_shape = Some(id);
                handler.enter_text_edit(canvas, text_id);
                return;
            }
        } else {
            let mut text = Text::new(world_point, String::new());
            text.style = canvas.tool_manager.current_style.clone();
            let shape = Shape::Text(text);
            let shape_id = shape.id();
            canvas.document.push_undo();
            canvas.document.add_shape(shape);
            canvas.clear_selection();
            canvas.select(shape_id);
            handler.enter_text_edit(canvas, shape_id);
            return;
        }

        let handle_tolerance = HANDLE_HIT_TOLERANCE / canvas.camera.zoom;
        for &shape_id in &canvas.selection {
            if let Some(shape) = canvas.document.get_shape(shape_id) {
                if let Some(HandleKind::Rotate) =
                    hit_test_handles(shape, world_point, handle_tolerance)
                {
                    canvas.document.push_undo();
                    if let Some(mut shape) = canvas.document.get_shape_mut(shape_id) {
                        shape.set_rotation(0.0);
                    }
                    return;
                }
            }
        }
    }

    let handle_tolerance = HANDLE_HIT_TOLERANCE / canvas.camera.zoom;

    for &shape_id in &canvas.selection {
        if let Some(shape) = canvas.document.get_shape(shape_id) {
            if let Some(handle_kind) = hit_test_handles(shape, world_point, handle_tolerance) {
                handler.manipulation = Some(ManipulationState::new(
                    shape_id,
                    Some(handle_kind),
                    world_point,
                    shape.clone(),
                ));
                return;
            }
        }
    }

    let hits = canvas
        .document
        .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
    if let Some(&id) = hits.first() {
        if input.shift() {
            if canvas.is_selected(id) {
                canvas.selection.retain(|&s| s != id);
            } else {
                canvas.add_to_selection(id);
            }
        } else {
            if !canvas.is_selected(id) {
                canvas.clear_selection();
                canvas.select(id);
            }

            let mut original_shapes = HashMap::new();
            for &shape_id in &canvas.selection {
                if let Some(shape) = canvas.document.get_shape(shape_id) {
                    original_shapes.insert(shape_id, shape.clone());
                }
            }

            if !original_shapes.is_empty() {
                if input.alt() {
                    let mut mm = MultiMoveState::new_duplicate(world_point, original_shapes);
                    for shape in mm.original_shapes.values() {
                        let mut new_shape = shape.clone();
                        new_shape.regenerate_id();
                        let new_id = new_shape.id();
                        mm.duplicated_ids.push(new_id);
                        canvas.document.add_shape(new_shape);
                    }
                    canvas.clear_selection();
                    for &new_id in &mm.duplicated_ids {
                        canvas.add_to_selection(new_id);
                    }
                    handler.multi_move = Some(mm);
                } else {
                    handler.multi_move = Some(MultiMoveState::new(world_point, original_shapes));
                }
            }
        }
    } else {
        if !input.shift() {
            canvas.clear_selection();
        }
        handler.selection_rect = Some(SelectionRect {
            start: world_point,
            current: world_point,
        });
    }
}
