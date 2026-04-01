use kurbo::Affine;

use crate::canvas::Canvas;
use crate::event_handler::EventHandler;

impl EventHandler {
    pub(super) fn release_multi_move(&mut self, canvas: &mut Canvas) -> bool {
        let Some(mm) = self.multi_move.take() else {
            return false;
        };

        let delta = mm.delta();

        if mm.is_duplicate {
            if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
                canvas.document.push_undo();
                let translation = Affine::translate(delta);
                for (idx, &dup_id) in mm.duplicated_ids.iter().enumerate() {
                    let original_shape = mm.original_shapes.values().nth(idx);
                    if let Some(orig) = original_shape {
                        let mut new_shape = orig.clone();
                        new_shape.transform(translation);
                        if let Some(mut shape) = canvas.document.get_shape_mut(dup_id) {
                            *shape = new_shape;
                        }
                    }
                }
            } else {
                for &dup_id in &mm.duplicated_ids {
                    canvas.document.remove_shape(dup_id);
                }
                canvas.clear_selection();
                for &id in mm.original_shapes.keys() {
                    canvas.add_to_selection(id);
                }
            }
        } else if delta.x.abs() > 0.1 || delta.y.abs() > 0.1 {
            for (shape_id, original_shape) in &mm.original_shapes {
                if let Some(mut shape) = canvas.document.get_shape_mut(*shape_id) {
                    *shape = original_shape.clone();
                }
            }

            canvas.document.push_undo();
            let translation = Affine::translate(delta);
            for (shape_id, original_shape) in &mm.original_shapes {
                let mut new_shape = original_shape.clone();
                new_shape.transform(translation);
                if let Some(mut shape) = canvas.document.get_shape_mut(*shape_id) {
                    *shape = new_shape;
                }
            }
        }

        true
    }
}
