use crate::canvas::Canvas;
use crate::input::InputState;
use crate::shapes::ShapeStyle;
use kurbo::Point;

use crate::event_handler::EventHandler;

impl EventHandler {
    pub fn handle_release(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        input: &InputState,
        current_style: &ShapeStyle,
        grid_snap_enabled: bool,
        angle_snap_enabled: bool,
    ) {
        self.rotation_state = None;

        if self.release_manipulation(canvas, input) {
            return;
        }
        if self.release_multi_move(canvas) {
            return;
        }
        if self.release_selection_rect(canvas, input) {
            return;
        }
        self.release_tools(
            canvas,
            world_point,
            input,
            current_style,
            grid_snap_enabled,
            angle_snap_enabled,
        );
    }
}
