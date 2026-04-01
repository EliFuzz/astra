use crate::canvas::Canvas;
use crate::input::InputState;
use crate::tools::ToolKind;
use kurbo::Point;

use super::EventHandler;
use super::press_editing;
use super::press_misc;
use super::press_select_tool;
use super::press_text_tool;

impl EventHandler {
    pub fn handle_press(
        &mut self,
        canvas: &mut Canvas,
        world_point: Point,
        input: &InputState,
        grid_snap_enabled: bool,
    ) {
        if press_editing::short_circuit_if_editing_text(self, canvas, world_point) {
            return;
        }

        let tool = canvas.tool_manager.current_tool;
        match tool {
            ToolKind::Text => press_text_tool::handle(self, canvas, world_point),
            ToolKind::Select => press_select_tool::handle(self, canvas, world_point, input),
            _ => press_misc::handle(self, canvas, world_point, grid_snap_enabled, tool),
        }
    }
}
