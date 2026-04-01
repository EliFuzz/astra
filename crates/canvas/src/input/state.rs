use crate::platform::Instant;
use kurbo::Point;
use winit::keyboard::ModifiersState;
use winit_input_helper::WinitInputHelper;

use super::touch_state::TouchState;

pub struct InputState {
    pub(crate) helper: WinitInputHelper,
    pub(crate) last_click_time: Option<Instant>,
    pub(crate) last_click_position: Option<Point>,
    pub(crate) click_count: u32,
    pub(crate) double_click_detected: bool,
    pub(crate) triple_click_detected: bool,
    pub is_dragging: bool,
    pub drag_start: Option<Point>,
    pub(crate) touches: [Option<TouchState>; 2],
    pub(crate) pinch_distance: Option<f64>,
    pub(crate) pinch_center: Option<Point>,
    pub(crate) active_modifiers: ModifiersState,
}
