use crate::canvas::Canvas;
use crate::input::InputState;

use crate::event_handler::EventHandler;

impl EventHandler {
    pub(super) fn release_selection_rect(
        &mut self,
        canvas: &mut Canvas,
        input: &InputState,
    ) -> bool {
        let Some(sel_rect) = self.selection_rect.take() else {
            return false;
        };

        let rect = sel_rect.to_rect();
        if rect.width() > 2.0 && rect.height() > 2.0 {
            let shapes_in_rect = canvas.document.shapes_in_rect(rect);
            if !input.shift() {
                canvas.clear_selection();
            }
            for id in shapes_in_rect {
                canvas.add_to_selection(id);
            }
        }
        true
    }
}
