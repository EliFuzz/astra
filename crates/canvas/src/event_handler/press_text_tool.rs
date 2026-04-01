use std::collections::HashMap;

use crate::canvas::Canvas;
use crate::selection::{
    HANDLE_HIT_TOLERANCE, ManipulationState, MultiMoveState, hit_test_boundary,
    hit_test_handles,
};
use crate::shapes::Shape;
use kurbo::Point;

use super::EventHandler;

pub(super) fn handle(handler: &mut EventHandler, canvas: &mut Canvas, world_point: Point) {
    let hits = canvas
        .document
        .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
    let Some(&id) = hits.first() else {
        return;
    };
    let Some(shape @ Shape::Text(_)) = canvas.document.get_shape(id) else {
        return;
    };
    let boundary_tolerance = 8.0 / canvas.camera.zoom;
    let handle_tolerance = HANDLE_HIT_TOLERANCE / canvas.camera.zoom;

    if let Some(handle_kind) = hit_test_handles(shape, world_point, handle_tolerance) {
        handler.manipulation = Some(ManipulationState::new(
            id,
            Some(handle_kind),
            world_point,
            shape.clone(),
        ));
        canvas.clear_selection();
        canvas.select(id);
        return;
    }

    if hit_test_boundary(shape, world_point, boundary_tolerance) {
        let mut original_shapes = HashMap::new();
        original_shapes.insert(id, shape.clone());
        handler.multi_move = Some(MultiMoveState::new(world_point, original_shapes));
        canvas.clear_selection();
        canvas.select(id);
        return;
    }

    handler.enter_text_edit(canvas, id);
    canvas.clear_selection();
    canvas.select(id);
}
