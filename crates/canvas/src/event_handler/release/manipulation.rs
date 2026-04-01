use crate::canvas::Canvas;
use crate::input::InputState;
use crate::selection::{HandleKind, apply_manipulation};
use crate::shapes::Shape;

use crate::event_handler::EventHandler;

use super::arrow_finalize::finalize_arrow_route;

impl EventHandler {
    pub(super) fn release_manipulation(&mut self, canvas: &mut Canvas, input: &InputState) -> bool {
        let Some(manip) = self.manipulation.take() else {
            return false;
        };

        if matches!(manip.handle, Some(HandleKind::Rotate)) {
            if let Some(shape) = canvas.document.get_shape(manip.shape_id) {
                let current_rotation = shape.rotation();
                let original_rotation = manip.original_shape.rotation();

                if (current_rotation - original_rotation).abs() > 0.001 {
                    if let Some(mut shape) = canvas.document.get_shape_mut(manip.shape_id) {
                        *shape = manip.original_shape.clone();
                    }
                    canvas.document.push_undo();
                    if let Some(mut shape) = canvas.document.get_shape_mut(manip.shape_id) {
                        shape.set_rotation(current_rotation);
                    }
                }
            }
            return true;
        }

        let delta = manip.delta();

        if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
            if let Some(mut shape) = canvas.document.get_shape_mut(manip.shape_id) {
                *shape = manip.original_shape.clone();
            }
            canvas.document.push_undo();
            let mut new_shape =
                apply_manipulation(&manip.original_shape, manip.handle, delta, input.shift());

            let is_arrow_endpoint = matches!(&new_shape, Shape::Arrow(_))
                && matches!(manip.handle, Some(HandleKind::Endpoint(_)));

            if is_arrow_endpoint {
                if let Shape::Arrow(arrow) = &mut new_shape {
                    if let Some(sb) = manip.pending_start_binding {
                        arrow.start_binding = sb;
                    }
                    if let Some(eb) = manip.pending_end_binding {
                        arrow.end_binding = eb;
                    }
                    finalize_arrow_route(arrow, manip.shape_id, canvas);
                }
            }

            if let Some(mut shape) = canvas.document.get_shape_mut(manip.shape_id) {
                *shape = new_shape;
            }
        }
        true
    }
}
