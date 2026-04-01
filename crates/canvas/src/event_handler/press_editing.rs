use crate::canvas::Canvas;
use crate::shapes::Shape;
use kurbo::Point;

use super::EventHandler;

pub(super) fn short_circuit_if_editing_text(
    handler: &mut EventHandler,
    canvas: &mut Canvas,
    world_point: Point,
) -> bool {
    let editing_id = match handler.editing_text {
        Some(id) => id,
        None => return false,
    };
    let hits = canvas
        .document
        .shapes_at_point(world_point, 5.0 / canvas.camera.zoom);
    let clicked_on_editing = hits.first().map(|&id| {
        if id == editing_id {
            return true;
        }
        if let Some(Shape::Group(g)) = canvas.document.get_shape(id) {
            return g.children.iter().any(|c| c.id() == editing_id);
        }
        false
    }).unwrap_or(false);
    if clicked_on_editing {
        return true;
    }
    handler.exit_text_edit(canvas);
    false
}
